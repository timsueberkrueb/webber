use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;

use qmetaobject::*;

#[allow(non_snake_case)]
#[derive(QObject, Default)]
pub struct IconModel {
    base: qt_base_class!(trait QAbstractListModel),
    add: qt_method!(fn(&mut self, url: String)),
    clear: qt_method!(fn(&mut self)),
    get: qt_method!(fn(&self, idx: i32) -> QString),
    modelChanged: qt_signal!(),
    pub image_loaded: qt_method!(fn(&mut self, url: String, size: QSize)),
    pub add_image: qt_signal!(url: String),
    sizes: HashMap<String, QSize>,
    sorted: Vec<String>,
}

impl IconModel {
    pub fn construct() -> RefCell<Self> {
        let cell = RefCell::new(Self::default());
        unsafe { QObject::cpp_construct(&cell) };
        cell
    }

    fn add(&mut self, url: String) {
        self.add_image(url);
    }

    fn clear(&mut self) {
        self.begin_reset_model();
        self.sorted.clear();
        self.sizes.clear();
        self.end_reset_model();
        self.modelChanged();
    }

    fn get(&self, idx: i32) -> QString {
        self.sorted
            .get(idx as usize)
            .map(|s| QString::from(s.as_str()))
            .unwrap_or_default()
    }

    pub fn image_loaded(&mut self, url: String, size: QSize) {
        // Ignore images that aren't quadratic for now.
        // TODO: Add an option to crop non-quadratic images
        if !is_quadratic(&size) || is_unreasonable(&size) {
            return;
        }
        self.sizes.insert(url.clone(), size);
        let idx = self.new_index_for(&url);
        self.begin_insert_rows(idx as i32, idx as i32);
        self.sorted.insert(idx, url);
        self.end_insert_rows();
        self.modelChanged();
    }

    fn new_index_for(&self, url: &str) -> usize {
        // TODO: Also consider the icon type when sorting
        // (e.g. from PWA metadata > Apple touch icon > opengraph image > favicon)
        let size = &self.sizes[url];
        self.sorted
            .binary_search_by(|other| {
                let other_size = &self.sizes[other];
                // Prefer SVGs
                url.to_lowercase()
                    .ends_with(".svg")
                    .cmp(&other.to_lowercase().ends_with(".svg"))
                    .then(cmp_qsize(size, other_size))
            })
            .collapse_into()
    }
}

impl QAbstractListModel for IconModel {
    fn row_count(&self) -> i32 {
        self.sorted.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let idx = index.row();
        if idx >= 0 && (idx as usize) < self.sorted.len() {
            let url = &self.sorted[idx as usize];
            let size = self.sizes[url];
            return if role == USER_ROLE {
                QVariant::from(QString::from(url.clone()))
            } else if role == USER_ROLE + 1 {
                QVariant::from(size.width)
            } else if role == USER_ROLE + 2 {
                QVariant::from(size.height)
            } else {
                QVariant::default()
            };
        }

        QVariant::default()
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();
        map.insert(USER_ROLE, QByteArray::from("url"));
        map.insert(USER_ROLE + 1, QByteArray::from("sourceWidth"));
        map.insert(USER_ROLE + 2, QByteArray::from("sourceHeight"));
        map
    }
}

trait CollapseInto<T> {
    fn collapse_into(self) -> T;
}

impl<T> CollapseInto<T> for Result<T, T> {
    fn collapse_into(self) -> T {
        match self {
            Ok(x) => x,
            Err(x) => x,
        }
    }
}

fn is_unreasonable(size: &QSize) -> bool {
    size.width == 0 || size.height == 0 || size.width > 2048 || size.height > 2048
}

fn is_quadratic(size: &QSize) -> bool {
    size.width == size.height
}

fn cmp_qsize(this: &QSize, other: &QSize) -> cmp::Ordering {
    this.width
        .cmp(&other.width)
        .then(this.height.cmp(&other.height))
}
