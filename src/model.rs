use std::cell::RefCell;

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
    create: qt_method!(fn(&mut self, url: String, name: String, theme_color: String, icon_url: String, url_patterns: String)),
    created: qt_signal!(),
}

impl AppModel {
    fn create(
        &mut self,
        url: String,
        name: String,
        theme_color: String,
        icon_url: String,
        url_patterns: String,
    ) {
        let package = click::Package {
            url,
            name,
            icon_url,
            theme_color,
            url_patterns,
        };

        let qptr = QPointer::from(&*self);
        let set_created = qmetaobject::queued_callback(move |_| {
            if let Some(self_) = qptr.as_pinned() {
                self_.borrow().created();
            }
        });

        std::thread::spawn(move || {
            click::create_package(package).unwrap();
            set_created(());
        });
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
    getPatternsString: qt_method!(fn(&mut self) -> QString),
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

    #[allow(non_snake_case)]
    fn getPatternsString(&mut self) -> QString {
        let s = self
            .model
            .borrow()
            .iter()
            .map(|pat| pat.url.clone())
            .collect::<Vec<_>>()
            .join(",");
        QString::from(s)
    }
}
