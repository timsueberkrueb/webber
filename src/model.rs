use std::cell::RefCell;
use std::path::PathBuf;

use csscolorparser::Color;
use url::Url;

use qmetaobject::*;

use crate::click;
use crate::fetch_and_resolve::*;
use crate::pwa::Manifest;
use crate::scraper::{self, ScrapedSite};

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct WebScraper {
    base: qt_base_class!(trait QObject),
    url: qt_property!(QString; NOTIFY urlChanged),
    urlChanged: qt_signal!(),
    siteName: qt_property!(QString; NOTIFY scraped),
    title: qt_property!(QString; NOTIFY scraped),
    themeColor: qt_property!(QString; NOTIFY scraped),
    iconUrl: qt_property!(QString; NOTIFY scraped),
    defaultUrlPatterns: qt_property!(QVariant; NOTIFY scraped),
    scraped: qt_signal!(),
    busy: qt_property!(bool; NOTIFY busyChanged),
    busyChanged: qt_signal!(),
    errorString: qt_property!(QString; NOTIFY errorStringChanged),
    errorStringChanged: qt_signal!(),
    scrape: qt_method!(fn(&mut self)),
    scrape_mutex: std::sync::Arc<std::sync::Mutex<()>>,
}

impl WebScraper {
    fn scrape(&mut self) {
        let url = self.url.to_string();

        match scraper::validate_url(url) {
            Ok(url) => {
                self.errorString = QString::default();
                self.errorStringChanged();
                self.run_scrape_thread(url);
            }
            Err(s) => {
                let msg = format!("Invalid url: {}", s);
                let msg = QString::from(msg);
                self.errorString = msg;
                self.errorStringChanged();
            }
        }
    }

    fn run_scrape_thread(&mut self, url: url::Url) {
        self.busy = true;
        self.busyChanged();

        let qptr = QPointer::from(&*self);
        let set_error_string = qmetaobject::queued_callback(move |val: QString| {
            if let Some(self_) = qptr.as_pinned() {
                self_.borrow_mut().errorString = val;
                self_.borrow().errorStringChanged();
            }
        });
        let qptr = QPointer::from(&*self);
        let set_scrape_result = qmetaobject::queued_callback(move |res: ScrapedSite<Resolved>| {
            if let Some(self_) = qptr.as_pinned() {
                let preferred_icon = res.icons.first();
                let white = Color::from_rgb_u8(255, 255, 255);

                self_.borrow_mut().title = QString::from(res.title.unwrap_or_default());
                self_.borrow_mut().siteName = QString::from(res.site_name.unwrap_or_default());
                self_.borrow_mut().themeColor =
                    QString::from(res.theme_color.unwrap_or(white).to_hex_string());
                self_.borrow_mut().iconUrl =
                    QString::from(preferred_icon.map(Url::as_str).unwrap_or_default());
                let mut list = QVariantList::default();
                for pat in res.default_url_patterns {
                    list.push(QVariant::from(QString::from(pat)));
                }
                self_.borrow_mut().defaultUrlPatterns = QVariant::from(list);
                self_.borrow().scraped();
            }
        });
        let qptr = QPointer::from(&*self);
        let set_busy = qmetaobject::queued_callback(move |busy| {
            if let Some(self_) = qptr.as_pinned() {
                self_.borrow_mut().busy = busy;
                self_.borrow().busyChanged();
            }
        });

        let mutex = self.scrape_mutex.clone();

        std::thread::spawn(move || {
            if let Ok(lock) = mutex.try_lock() {
                match ScrapedSite::fetch_and_resolve(&url) {
                    Ok(mut res) => {
                        let m = res
                            .manifest_url
                            .as_ref()
                            .map(|url| Manifest::fetch_and_resolve(url))
                            .map(|m| m.ok())
                            .flatten();

                        if let Some(m) = m {
                            res = res.supplemented(m);
                        }

                        set_scrape_result(res);
                    }
                    Err(err) => {
                        let msg = format!("Failed to load site: {}", err);
                        set_error_string(QString::from(msg));
                    }
                };
                drop(lock)
            }
            set_busy(false);
        });
    }
}

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct AppModel {
    base: qt_base_class!(trait QObject),
    create: qt_method!(fn(&mut self)),
    created: qt_signal!(),
    urlPatterns: qt_property!(RefCell<UrlPatterns>; CONST),
    permissions: qt_property!(RefCell<Permissions>; CONST),
    url: qt_property!(String),
    name: qt_property!(String),
    themeColor: qt_property!(String),
    iconUrl: qt_property!(String),
    enableAddressBar: qt_property!(bool),
    enableBackForward: qt_property!(bool),
    enableFullscreen: qt_property!(bool),
    enableDesktopUserAgent: qt_property!(bool),
    clickPath: qt_property!(String; NOTIFY clickPathChanged),
    clickPathChanged: qt_signal!(),
    screenshotIconPath: qt_property!(String; READ screenshot_icon_path),
    customIconPath: qt_property!(String),
    useScreenshotIcon: qt_property!(bool),
    useCustomIcon: qt_property!(bool),
}

impl AppModel {
    fn create(&mut self) {
        let mut package = click::Package {
            url: self.url.clone(),
            name: self.name.clone(),
            icon: if self.useCustomIcon {
                click::Icon::Local(self.customIconPath.clone())
            } else if self.useScreenshotIcon {
                click::Icon::Local(self.screenshot_icon_path())
            } else {
                click::Icon::Remote(self.iconUrl.clone())
            },
            theme_color: self.themeColor.clone(),
            url_patterns: self.urlPatterns.borrow().get_patterns_string(),
            permissions: self.permissions.borrow().get_enabled(),
            enable_address_bar: self.enableAddressBar,
            enable_back_forward: self.enableBackForward,
            enable_fullscreen: self.enableFullscreen,
            enable_desktop_user_agent: self.enableDesktopUserAgent,
        };
        package.sanitize();

        let qptr = QPointer::from(&*self);
        let set_created = qmetaobject::queued_callback(move |path: PathBuf| {
            if let Some(self_) = qptr.as_pinned() {
                self_.borrow_mut().clickPath = path.to_str().unwrap().to_owned();
                self_.borrow_mut().clickPathChanged();
                self_.borrow().created();
            }
        });

        std::thread::spawn(move || {
            let path = package.create().unwrap();
            set_created(path);
        });
    }

    fn screenshot_icon_path(&self) -> String {
        let path = xdg::BaseDirectories::new()
            .unwrap()
            .get_cache_home()
            .join("webber.timsueberkrueb");
        path.join("screenshot-icon.png")
            .to_str()
            .unwrap()
            .to_owned()
    }
}

#[derive(Default, Clone, SimpleListItem)]
pub struct UrlPattern {
    pub url: String,
}

pub type UrlPatternsModel = SimpleListModel<UrlPattern>;

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct UrlPatterns {
    base: qt_base_class!(trait QObject),
    model: qt_property!(RefCell<UrlPatternsModel>; CONST),
    setUrl: qt_method!(fn(&mut self, idx: usize, url: String) -> bool),
    add: qt_method!(fn(&mut self, url: String)),
    remove: qt_method!(fn(&mut self, index: usize) -> bool),
    clear: qt_method!(fn(&mut self)),
}

impl UrlPatterns {
    #[allow(non_snake_case)]
    fn setUrl(&mut self, row: usize, url: String) -> bool {
        let mut model = self.model.borrow_mut();

        if row > model.row_count() as usize {
            return false;
        }
        model.change_line(row, UrlPattern { url });
        true
    }

    fn add(&mut self, url: String) {
        let mut model = self.model.borrow_mut();
        model.push(UrlPattern { url });
    }

    fn remove(&mut self, row: usize) -> bool {
        let mut model = self.model.borrow_mut();
        if row > model.row_count() as usize {
            return false;
        }
        model.remove(row);
        true
    }

    fn clear(&mut self) {
        let mut model = self.model.borrow_mut();
        model.reset_data(Vec::default());
    }

    fn get_patterns_string(&self) -> String {
        self.model
            .borrow()
            .iter()
            .map(|pat| pat.url.clone())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[derive(Default, Clone, SimpleListItem)]
pub struct Permission {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

impl Permission {
    fn new(name: String, description: String, enabled: bool) -> Self {
        Self {
            name,
            description,
            enabled,
        }
    }
}

pub type PermissionsModel = SimpleListModel<Permission>;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct Permissions {
    base: qt_base_class!(trait QObject),
    model: qt_property!(RefCell<PermissionsModel>; CONST),
    add: qt_method!(fn(&mut self, name: String, description: String, default: bool)),
    clear: qt_method!(fn(&mut self)),
    setEnabled: qt_method!(fn(&mut self, row: usize, enabled: bool) -> bool),
}

impl Permissions {
    fn get_enabled(&self) -> Vec<String> {
        self.model
            .borrow()
            .iter()
            .filter_map(|perm| {
                if perm.enabled {
                    Some(perm.name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn add(&mut self, name: String, description: String, default: bool) {
        self.model
            .borrow_mut()
            .push(Permission::new(name, description, default));
    }

    pub fn clear(&mut self) {
        self.model.borrow_mut().reset_data(Vec::new());
    }

    #[allow(non_snake_case)]
    fn setEnabled(&mut self, row: usize, enabled: bool) -> bool {
        let mut model = self.model.borrow_mut();
        if row > model.row_count() as usize {
            return false;
        }
        let mut perm = model[row].clone();
        perm.enabled = enabled;
        model.change_line(row, perm);
        true
    }
}
