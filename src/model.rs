use std::collections::HashMap;

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

#[derive(Default, Clone)]
struct UrlPattern {
    url: String,
}

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct UrlPatternsModel {
    base: qt_base_class!(trait QAbstractListModel),
    count: qt_property!(i32; READ row_count NOTIFY count_changed),
    count_changed: qt_signal!(),
    list: Vec<UrlPattern>,

    setUrl: qt_method!(fn(&mut self, idx: usize, url: String) -> bool),
    insert_row: qt_method!(fn(&mut self, row: usize) -> bool),
    remove_row: qt_method!(fn(&mut self, row: usize) -> bool),
    add: qt_method!(fn(&mut self, url: String)),
    remove: qt_method!(fn(&mut self, index: u64) -> bool),
    clear: qt_method!(fn(&mut self)),
    getPatternsString: qt_method!(fn(&mut self) -> QString),
}

impl UrlPatternsModel {
    #[allow(non_snake_case)]
    fn setUrl(&mut self, idx: usize, url: String) -> bool {
        if idx > self.list.len() {
            return false;
        }
        self.list[idx].url = url;
        let idx = (self as &mut dyn QAbstractListModel).row_index(idx as i32);
        (self as &mut dyn QAbstractListModel).data_changed(idx, idx);
        true
    }

    fn add(&mut self, url: String) {
        let end = self.list.len();
        (self as &mut dyn QAbstractListModel).begin_insert_rows(end as i32, end as i32);
        self.list.insert(end, UrlPattern { url });
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
    }

    fn remove(&mut self, index: u64) -> bool {
        self.remove_row(index as usize)
    }

    fn clear(&mut self) {
        (self as &mut dyn QAbstractListModel).begin_reset_model();
        self.list.clear();
        (self as &mut dyn QAbstractListModel).end_reset_model();
        self.count_changed();
    }

    #[allow(non_snake_case)]
    fn getPatternsString(&mut self) -> QString {
        let s = self
            .list
            .iter()
            .map(|pat| pat.url.clone())
            .collect::<Vec<_>>()
            .join(",");
        QString::from(s)
    }

    fn insert_row(&mut self, row: usize) -> bool {
        if row > self.list.len() {
            return false;
        }
        (self as &mut dyn QAbstractListModel).begin_insert_rows(row as i32, (row + 1) as i32);
        self.list.insert(row, UrlPattern::default());
        (self as &mut dyn QAbstractListModel).end_insert_rows();
        self.count_changed();
        true
    }

    fn remove_row(&mut self, row: usize) -> bool {
        if row > self.list.len() {
            return false;
        }
        (self as &mut dyn QAbstractListModel).begin_remove_rows(row as i32, row as i32);
        self.list.remove(row);
        (self as &mut dyn QAbstractListModel).end_remove_rows();
        self.count_changed();
        true
    }
}

impl QAbstractListModel for UrlPatternsModel {
    fn row_count(&self) -> i32 {
        self.list.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let idx = index.row() as usize;
        if idx < self.list.len() {
            if role == USER_ROLE {
                QString::from(self.list[idx].url.clone()).into()
            } else {
                QVariant::default()
            }
        } else {
            QVariant::default()
        }
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();
        map.insert(USER_ROLE, "url".into());
        map
    }
}
