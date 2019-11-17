use std::string::ToString;
use url::Url;

pub fn validate_url(url: String) -> Result<Url, String> {
    let url = Url::parse(&url).map_err(|err| err.to_string())?;
    Ok(url)
}

pub fn scrape_url(url: Url) -> Result<ScrapeResult, String> {
    let body = reqwest::get(url.as_ref())
        .map_err(|err| err.to_string())?
        .text()
        .map_err(|err| err.to_string())?;

    let html = scraper::Html::parse_document(&body);

    let res = ScrapeResult::parse(url, html);

    Ok(res)
}

pub struct ScrapeResult {
    pub site_name: String,
    pub title: String,
    pub theme_color: String,
    pub icon_url: String,
    pub default_url_patterns: Vec<String>,
}

impl ScrapeResult {
    pub fn parse(url: Url, html: scraper::Html) -> Self {
        let title_sel = scraper::Selector::parse("html > head > title").unwrap();
        let title = html
            .select(&title_sel)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_default();
        let og_name_sel =
            scraper::Selector::parse("html > head > meta[property='og:site_name']").unwrap();
        let site_name = html
            .select(&og_name_sel)
            .next()
            .map(|el| el.value().attr("content").unwrap_or_default().to_owned())
            .unwrap_or_default();
        let theme_color_sel =
            scraper::Selector::parse("html > head > meta[name='theme-color']").unwrap();
        let theme_color = html
            .select(&theme_color_sel)
            .next()
            .map(|el| el.value().attr("content").unwrap_or_default().to_owned())
            .unwrap_or_default();
        let icon_sel = scraper::Selector::parse("html > head > link[rel='icon']").unwrap();
        let apple_icon_sel =
            scraper::Selector::parse("html > head > link[rel='apple-touch-icon']").unwrap();
        let mut icon_url = html
            .select(&icon_sel)
            .next()
            .map(|el| el.value().attr("href").unwrap_or_default().to_owned())
            .unwrap_or_else(|| {
                html.select(&apple_icon_sel)
                    .next()
                    .map(|el| el.value().attr("href").unwrap_or_default().to_owned())
                    .unwrap_or_default()
            });
        if icon_url != "" {
            if let Ok(url) = url.join(&icon_url) {
                icon_url = url.to_string();
            }
        }

        let default_url_patterns = if let Some(url::Host::Domain(domain)) = url.host() {
            vec![
                format!("https?://{}/*", domain),
                format!("https?://*.{}/*", domain),
            ]
        } else {
            Vec::default()
        };

        Self {
            site_name,
            title,
            theme_color,
            icon_url,
            default_url_patterns,
        }
    }
}
