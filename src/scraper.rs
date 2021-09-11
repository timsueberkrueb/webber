use std::collections::HashSet;
use std::hash::Hash;
use std::string::ToString;

use csscolorparser::Color;
use reqwest::blocking as reqwest;
use url::Url;

use crate::fetchable::*;
use crate::pwa;
use crate::resolvable::*;
use crate::serde_utils::LossyVec;

pub struct ScrapedSite<R: ResolveType> {
    pub site_name: Option<String>,
    pub title: Option<String>,
    pub theme_color: Option<Color>,
    pub icons: R::Array<R::Url>,
    pub default_url_patterns: Vec<String>,
    pub manifest_url: Option<R::Url>,
}

impl ScrapedSite<Resolved> {
    pub fn supplemented(self, m: pwa::Manifest<Resolved>) -> ScrapedSite<Resolved> {
        let mut icons = m
            .icons
            .unwrap_or_default()
            .into_iter()
            .rev()
            .map(|icon| icon.src)
            .collect::<Vec<_>>();
        icons.extend(self.icons.into_iter());
        icons.prune_duplicates();

        let mut url_patterns = self.default_url_patterns;

        if let Some(scope_url) = m.scope {
            let scope_url = scope_url.join("/").unwrap_or(scope_url);
            if let Some(host_str) = scope_url.host_str() {
                let scope_str = format!("https?://{}{}*", host_str, scope_url.path());
                // Some sites' scope is actually insufficient (e.g. their scope is https://example.com/ but
                // they redirect to https://m.example.com). Hence we don't replace the default patterns,
                // but instead just add the scope pattern:
                if !url_patterns.contains(&scope_str) {
                    url_patterns.push(scope_str);
                }
            }
        }

        ScrapedSite {
            site_name: m.short_name.or(self.site_name),
            title: m.name.or(self.title),
            theme_color: m.theme_color.or(self.theme_color),
            icons,
            default_url_patterns: url_patterns,
            manifest_url: self.manifest_url,
        }
    }
}

impl Fetchable for ScrapedSite<Unresolved> {
    type Out<R: ResolveType> = ScrapedSite<R>;
    type Error = String;

    fn fetch(url: &Url) -> Result<ScrapedSite<Unresolved>, String> {
        let body = reqwest::get(url.as_ref())
            .map_err(|err| err.to_string())?
            .text()
            .map_err(|err| err.to_string())?;

        let html = scraper::Html::parse_document(&body);

        let res = Self::parse(url, html);

        Ok(res)
    }
}

impl ScrapedSite<Unresolved> {
    pub fn parse(url: &Url, html: scraper::Html) -> Self {
        let title_sel = scraper::Selector::parse("html > head > title").unwrap();
        let title = html
            .select(&title_sel)
            .next()
            .map(|el| el.inner_html())
            .map(|s| s.trim().to_owned());

        let og_name_sel =
            scraper::Selector::parse("html > head > meta[property='og:site_name']").unwrap();
        let site_name = html
            .select(&og_name_sel)
            .next()
            .and_then(|el| el.value().attr("content").map(String::from))
            .map(|s| s.trim().to_owned());

        let theme_color_sel =
            scraper::Selector::parse("html > head > meta[name='theme-color']").unwrap();
        let theme_color = html
            .select(&theme_color_sel)
            .next()
            .and_then(|el| el.value().attr("content"))
            .and_then(|c| c.parse().ok());

        let favicon_sel = scraper::Selector::parse("html > head > link[rel~='icon']").unwrap();
        let shortcut_sel = scraper::Selector::parse("html > head > link[rel~='shortcut']").unwrap();
        let apple_icon_sel =
            scraper::Selector::parse("html > head > link[rel~='apple-touch-icon']").unwrap();
        let opengraph_sel =
            scraper::Selector::parse("html > head > meta[property~='og:image']").unwrap();

        let favicon_url = html
            .select(&favicon_sel)
            .next()
            .and_then(|el| el.value().attr("href").map(String::from));

        let shortcut_icon_url = html
            .select(&shortcut_sel)
            .next()
            .and_then(|el| el.value().attr("href").map(String::from));

        let apple_icon_url = html
            .select(&apple_icon_sel)
            .next()
            .and_then(|el| el.value().attr("href").map(String::from));

        let open_graph_images = html
            .select(&opengraph_sel)
            .map(|el| el.value().attr("content").map(String::from));

        let mut icons = vec![apple_icon_url];
        icons.push(shortcut_icon_url);
        icons.extend(open_graph_images);
        icons.push(favicon_url);
        icons.prune_duplicates();

        let icons = LossyVec::from(icons);

        let mut default_url_patterns = Vec::new();

        // Create a default url pattern for any url with a host part
        if let Some(host_str) = url.host_str() {
            default_url_patterns.push(format!("https?://{}/*", host_str));
        }

        // Create a default url pattern for subdomains for any url whose host part is a domain
        if let Some(url::Host::Domain(domain_str)) = url.host() {
            default_url_patterns.push(format!("https?://*.{}/*", domain_str));
        }

        let manifest_sel = scraper::Selector::parse("html > head > link[rel='manifest']").unwrap();
        let manifest_url = html
            .select(&manifest_sel)
            .next()
            .and_then(|el| el.value().attr("href").map(String::from));

        Self {
            site_name,
            title,
            theme_color,
            icons,
            default_url_patterns,
            manifest_url,
        }
    }
}

impl Resolvable for ScrapedSite<Unresolved> {
    type Out<R: ResolveType> = ScrapedSite<R>;

    fn resolve(self, base_url: &Url) -> ScrapedSite<Resolved> {
        ScrapedSite {
            site_name: self.site_name,
            title: self.title,
            theme_color: self.theme_color,
            icons: Resolvable::resolve(self.icons, base_url)
                .into_iter()
                .flatten()
                .collect(),
            default_url_patterns: self.default_url_patterns,
            manifest_url: self.manifest_url.resolve(base_url).flatten(),
        }
    }
}

pub fn validate_url(url: String) -> Result<Url, String> {
    let url = Url::parse(&url).map_err(|err| err.to_string())?;
    Ok(url)
}

trait PruneDuplicates {
    fn prune_duplicates(&mut self);
}

impl<T: Eq + Hash + Clone> PruneDuplicates for Vec<T> {
    fn prune_duplicates(&mut self) {
        let mut seen = HashSet::new();
        self.retain(|el| {
            if seen.contains(el) {
                false
            } else {
                seen.insert(el.clone());
                true
            }
        });
    }
}
