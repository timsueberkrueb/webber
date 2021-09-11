use cpp::*;
use qmetaobject::*;
use std::cell::RefCell;
use std::pin::Pin;

use std::collections::HashMap;

/// Extension trait for adding a `QQuickImageProvider` to a `QmlEngine`
pub trait AddImageProvider<P: QQuickImageProvider> {
    /// Wrapper around [`void QQmlEngine::addImageProvider(const QString &providerId, QQmlImageProviderBase *provider)`][method]
    ///
    /// # Wrapper-specific
    ///
    /// Specialized to `ImageType::Image`.
    ///
    /// [method]: https://doc.qt.io/qt-5/qqmlengine.html#addImageProvider
    fn add_image_provider(&mut self, provider_id: &str, provider: RefCell<P>);
}

impl<P: QQuickImageProvider> AddImageProvider<P> for QmlEngine {
    fn add_image_provider(&mut self, provider_id: &str, provider: RefCell<P>) {
        let qml_engine = self.cpp_ptr();
        let provider_id = QString::from(provider_id);
        let provider_ptr = provider.borrow().get_cpp_object();
        cpp!(unsafe [
            qml_engine as "QQmlEngine *",
            provider_id as "QString",
            provider_ptr as "Rust_QImageProvider *"
        ] {
            qml_engine->addImageProvider(provider_id, provider_ptr);
        });
        std::mem::forget(provider);
    }
}

/// Extension trait for adding a `QQuickPixmapProvider` to a `QmlEngine`
pub trait AddPixmapProvider<P: QQuickPixmapProvider> {
    /// Wrapper around [`void QQmlEngine::addImageProvider(const QString &providerId, QQmlImageProviderBase *provider)`][method]
    ///
    /// # Wrapper-specific
    ///
    /// Specialized to `ImageType::Pixmap`.
    ///
    /// [method]: https://doc.qt.io/qt-5/qqmlengine.html#addImageProvider
    fn add_pixmap_provider(&mut self, provider_id: &str, provider: RefCell<P>);
}

impl<P: QQuickPixmapProvider> AddPixmapProvider<P> for QmlEngine {
    fn add_pixmap_provider(&mut self, provider_id: &str, provider: RefCell<P>) {
        let qml_engine = self.cpp_ptr();
        let provider_id = QString::from(provider_id);
        let provider_ptr = provider.borrow().get_cpp_object();
        cpp!(unsafe [
            qml_engine as "QQmlEngine *",
            provider_id as "QString",
            provider_ptr as "Rust_QPixmapProvider *"
        ] {
            qml_engine->addImageProvider(provider_id, provider_ptr);
        });
        std::mem::forget(provider);
    }
}

/// A simple, `HashMap`-based pixmap provider
#[derive(QObject, Default)]
pub struct SimplePixmapProvider {
    base: qt_base_class!(trait QQuickPixmapProvider),
    map: HashMap<String, QPixmap>,
}

impl QQuickPixmapProvider for SimplePixmapProvider {
    fn request_pixmap(&self, id: &str, _requested_size: &QSize) -> (QSize, QPixmap) {
        let pixmap = self.map.get(id).cloned().unwrap_or_default();
        (pixmap.size(), pixmap)
    }
}

/// A simple, `HashMap`-based image provider
#[derive(QObject, Default)]
pub struct SimpleImageProvider {
    base: qt_base_class!(trait QQuickImageProvider),
    pub map: HashMap<String, QImage>,
}

impl SimpleImageProvider {
    #[allow(dead_code)]
    pub fn construct() -> RefCell<Self> {
        let cell = RefCell::new(Self::default());
        unsafe { QObject::cpp_construct(&cell) };
        cell
    }
}

impl QQuickImageProvider for SimpleImageProvider {
    fn request_image(&self, id: &str, _requested_size: &QSize) -> (QSize, QImage) {
        let image = self.map.get(id).cloned().unwrap_or_default();
        (image.size(), image)
    }
}

/// [`QQuickImageProvider`][class] specialized to `ImageType::Pixmap`
///
/// [class]: https://doc.qt.io/qt-5/qquickimageprovider.html
pub trait QQuickPixmapProvider: QObject {
    /// Wrapper around [`QPixmap QQuickImageProvider::requestPixmap(const QString &id, QSize *size, const QSize &requestedSize)`][method]
    ///
    /// # Wrapper-specific
    ///
    /// Returns a tuple of the original image size and the pixmap instead of providing a mutable size
    /// parameter.
    ///
    /// [method]: https://doc.qt.io/qt-5/qquickimageprovider.html#requestPixmap
    fn request_pixmap(
        &self,
        #[allow(unused_variables)] id: &str,
        #[allow(unused_variables)] requested_size: &QSize,
    ) -> (QSize, QPixmap) {
        Default::default()
    }

    /// Required for the implementation detail of the QObject custom derive
    fn get_object_description() -> &'static QObjectDescriptor
    where
        Self: Sized,
    {
        unsafe {
            &*cpp!([]-> *const QObjectDescriptor as "RustQObjectDescriptor const*" {
                return RustQObjectDescriptor::instance<Rust_QPixmapProvider>();
            })
        }
    }
}

/// [`QQuickImageProvider`][class] specialized to `ImageType::Image`
///
/// [class]: https://doc.qt.io/qt-5/qquickimageprovider.html
pub trait QQuickImageProvider: QObject {
    /// Wrapper around [`QImage QQuickImageProvider::requestImage(const QString &id, QSize *size, const QSize &requestedSize)`][method]
    ///
    /// # Wrapper-specific
    ///
    /// Returns a tuple of the original image size and the image instead of providing a mutable size
    /// parameter.
    ///
    /// [method]: https://doc.qt.io/qt-5/qquickimageprovider.html#requestImage
    fn request_image(
        &self,
        #[allow(unused_variables)] id: &str,
        #[allow(unused_variables)] requested_size: &QSize,
    ) -> (QSize, QImage) {
        Default::default()
    }

    /// Required for the implementation detail of the QObject custom derive
    fn get_object_description() -> &'static QObjectDescriptor
    where
        Self: Sized,
    {
        unsafe {
            &*cpp!([]-> *const QObjectDescriptor as "RustQObjectDescriptor const*" {
                return RustQObjectDescriptor::instance<Rust_QImageProvider>();
            })
        }
    }
}

cpp! {{
    #include <src/qt/qmetaobject_rust.hpp>
    #include <QtQuick/QQuickImageProvider>

    class QPixmapProvider : public QObject, public QQuickImageProvider
    {
        public:
            QPixmapProvider(): QObject(), QQuickImageProvider(ImageType::Pixmap) {}
    };

    struct Rust_QPixmapProvider : public RustObject<QPixmapProvider>
    {
        QPixmap requestPixmap(const QString &id, QSize *size, const QSize &requested_size) override
        {
            return rust!(Rust_QPixmapProvider_request_pixmap [
                rust_object: QObjectPinned<dyn QQuickPixmapProvider> as "TraitObject",
                id: &QString as "const QString &",
                size: *mut QSize as "QSize *",
                requested_size: &QSize as "const QSize &"
            ] -> QPixmap as "QPixmap" {
                let (orig_size, pixmap) = rust_object.borrow().request_pixmap(&id.to_string(), requested_size);
                if !size.is_null() {
                    *size = orig_size;
                }
                pixmap
            });
        }
    };

    class QImageProvider : public QObject, public QQuickImageProvider
    {
        public:
            QImageProvider(): QQuickImageProvider(ImageType::Image) {}
    };

    struct Rust_QImageProvider : public RustObject<QImageProvider>
    {
        QImage requestImage(const QString &id, QSize *size, const QSize &requested_size) override
        {
            return rust!(Rust_QImageProvider_request_image [
                rust_object: Pin<&RefCell<dyn QQuickImageProvider>> as "TraitObject",
                id: &QString as "const QString &",
                size: *mut QSize as "QSize *",
                requested_size: &QSize as "const QSize &"
            ] -> QImage as "QImage" {
                let (orig_size, img) = rust_object.borrow().request_image(&id.to_string(), requested_size);
                if !size.is_null() {
                    *size = orig_size;
                }
                img
            });
        }
    };
}}
