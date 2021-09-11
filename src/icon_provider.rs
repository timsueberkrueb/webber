use std::cell::RefCell;
use std::collections::HashMap;

use reqwest::blocking as reqwest;

use qmetaobject::*;

use crate::qt::image_provider::*;

/// A simple, `HashMap`-based image provider
#[derive(QObject, Default)]
pub struct IconProvider {
    base: qt_base_class!(trait QQuickPixmapProvider),
    pub add_image: qt_method!(fn(&mut self, url: String)),
    pub image_loaded: qt_signal!(url: String, size: QSize),
    pixmaps: HashMap<String, QPixmap>,
}

impl IconProvider {
    pub fn construct() -> RefCell<Self> {
        let cell = RefCell::new(Self::default());
        unsafe { QObject::cpp_construct(&cell) };
        cell
    }

    pub fn add_image(&mut self, url: String) {
        let qptr = QPointer::from(&*self);
        let url_clone = url.clone();

        let on_downloaded = qmetaobject::queued_callback(move |pixmap: QPixmap| {
            if let Some(self_) = qptr.as_pinned() {
                let size = pixmap.size();
                self_.borrow_mut().pixmaps.insert(url.clone(), pixmap);
                self_.borrow().image_loaded(url.clone(), size);
            }
        });

        std::thread::spawn(move || match download_icon(&url_clone) {
            Ok(pixmap) => on_downloaded(pixmap),
            Err(msg) => eprintln!("Failed to download icon {}", msg),
        });
    }
}

impl QQuickPixmapProvider for IconProvider {
    fn request_pixmap(&self, id: &str, requested_size: &QSize) -> (QSize, QPixmap) {
        let pixmap = self.pixmaps.get(id).cloned().unwrap_or_default();
        let pixmap = pixmap.scaled(
            *requested_size,
            AspectRatioMode::IgnoreAspectRatio,
            TransformationMode::SmoothTransformation,
        );
        (pixmap.size(), pixmap)
    }
}

fn download_icon(url: &str) -> Result<QPixmap, String> {
    let bytes = reqwest::get(url)
        .map_err(|err| err.to_string())?
        .bytes()
        .map_err(|err| err.to_string())?;

    let byte_array = QByteArray::from(&bytes[..]);
    let pixmap = QPixmap::load_from_bytearray(byte_array);

    Ok(pixmap)
}
