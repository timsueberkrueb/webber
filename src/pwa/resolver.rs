use url::Url;

use super::parser::*;

use crate::resolvable::*;

impl Resolvable for Manifest<Unresolved> {
    type Out<R: ResolveType> = Manifest<R>;

    fn resolve(self, base_url: &Url) -> Manifest<Resolved> {
        Manifest {
            background_color: self.background_color,
            dir: self.dir,
            display: self.display,
            icons: self.icons.map(|icons| {
                Resolvable::resolve(icons, base_url)
                    .into_iter()
                    .flatten()
                    .collect()
            }),
            lang: self.lang,
            name: self.name,
            orientation: self.orientation,
            scope: self.scope.resolve(base_url).flatten(),
            short_name: self.short_name,
            shortcuts: self.shortcuts.map(|s| {
                Resolvable::resolve(s, base_url)
                    .into_iter()
                    .flatten()
                    .collect()
            }),
            start_url: self.start_url.resolve(base_url).flatten(),
            theme_color: self.theme_color,
        }
    }
}

impl Resolvable for ShortcutItem<Unresolved> {
    type Out<R: ResolveType> = Option<ShortcutItem<R>>;

    fn resolve(self, base_url: &Url) -> Option<ShortcutItem<Resolved>> {
        Some(ShortcutItem {
            name: self.name,
            short_name: self.short_name,
            description: self.description,
            url: self.url.resolve(base_url)?,
            icons: self.icons.map(|icons| {
                Resolvable::resolve(icons, base_url)
                    .into_iter()
                    .flatten()
                    .collect()
            }),
        })
    }
}

impl Resolvable for Icon<Unresolved> {
    type Out<R: ResolveType> = Option<Icon<R>>;

    fn resolve(self, base_url: &Url) -> Self::Out<Resolved> {
        Some(Icon {
            sizes: self.sizes,
            src: self.src.resolve(base_url)?,
            icon_type: self.icon_type,
            purpose: self.purpose,
        })
    }
}
