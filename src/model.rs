use std::cell::RefCell;
use std::path::PathBuf;

use qmetaobject::*;

use crate::click;
use crate::core;

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

        match core::validate_url(url) {
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
        let set_scrape_result = qmetaobject::queued_callback(move |res: core::ScrapeResult| {
            if let Some(self_) = qptr.as_pinned() {
                self_.borrow_mut().title = QString::from(res.title);
                self_.borrow_mut().siteName = QString::from(res.site_name);
                self_.borrow_mut().themeColor = QString::from(res.theme_color);
                self_.borrow_mut().iconUrl = QString::from(res.icon_url);
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
                match core::scrape_url(url) {
                    Ok(res) => set_scrape_result(res),
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
    clickPath: qt_property!(String; NOTIFY clickPathChanged),
    clickPathChanged: qt_signal!(),
    screenshotIconPath: qt_property!(String; READ screenshot_icon_path),
    useScreenshotIcon: qt_property!(bool),
}

impl AppModel {
    fn create(&mut self) {
        let package = click::Package {
            url: self.url.clone(),
            name: self.name.clone(),
            icon: if self.useScreenshotIcon {
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
        };

        let qptr = QPointer::from(&*self);
        let set_created = qmetaobject::queued_callback(move |path: PathBuf| {
            if let Some(self_) = qptr.as_pinned() {
                self_.borrow_mut().clickPath = path.to_str().unwrap().to_owned();
                self_.borrow_mut().clickPathChanged();
                self_.borrow().created();
            }
        });

        std::thread::spawn(move || {
            let path = click::create_package(package).unwrap();
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
    fn new(name: &str, description: &str, enabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
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
    loadDefaults: qt_method!(fn(&mut self)),
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

    #[allow(non_snake_case)]
    pub fn loadDefaults(&mut self) {
        let perms = vec![
            Permission::new("audio", "Play audio", true),
            Permission::new("content_exchange", "Upload files from other apps", true),
            Permission::new(
                "content_exchange_source",
                "Export files to other apps",
                false,
            ),
            Permission::new("keep-display-on", "Keep the screen on", false),
            Permission::new("location", "Access your location", false),
            Permission::new("camera", "Access your camera", false),
            Permission::new("microphone", "Acess your microphone", false),
            Permission::new("sensores", "Access your sensors", false),
        ];
        self.model.borrow_mut().reset_data(perms);
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
