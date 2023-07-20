#![feature(generic_associated_types)]
#![recursion_limit = "256"]

#[macro_use]
extern crate cstr;
#[macro_use]
extern crate qmetaobject;

use std::env;
use std::path::PathBuf;

use gettextrs::{bindtextdomain, textdomain};
use qmetaobject::*;

mod click;
mod fetch_and_resolve;
mod fetchable;
mod icon_model;
mod icon_provider;
mod model;
mod pwa;
mod qrc;
mod qt;
mod resolvable;
mod scraper;
mod serde_utils;

use crate::icon_model::IconModel;
use crate::icon_provider::IconProvider;
use crate::qt::image_provider::AddPixmapProvider;

fn main() {
    init_gettext();

    std::env::set_var("QTWEBENGINE_CHROMIUM_FLAGS", "--disable-gpu --disable-viz-display-compositor --enable-features=OverlayScrollbar,OverlayScrollbarFlashAfterAnyScrollUpdate,OverlayScrollbarFlashWhenMouseEnter");

    unsafe {
        cpp! {[]{
            QtWebEngine::initialize();
        }}
    }

    unsafe {
        cpp! {{
            #include <QtCore/QCoreApplication>
            #include <QtCore/QString>
            #include <QtWebEngine/QtWebEngine>
        }}
        cpp! {[]{
            QCoreApplication::setAttribute(Qt::AA_ShareOpenGLContexts);
            QCoreApplication::setApplicationName(QStringLiteral("webber.timsueberkrueb"));
        }}
    }
    QQuickStyle::set_style("Suru");
    qrc::load();
    qml_register_type::<model::WebScraper>(cstr!("Webber"), 1, 0, cstr!("WebScraper"));
    qml_register_type::<model::AppModel>(cstr!("Webber"), 1, 0, cstr!("AppModel"));
    qml_register_type::<model::UrlPatternsModel>(cstr!("Webber"), 1, 0, cstr!("UrlPatternsModel"));
    qml_register_type::<model::UrlPatterns>(cstr!("Webber"), 1, 0, cstr!("UrlPatterns"));
    qml_register_type::<model::PermissionsModel>(cstr!("Webber"), 1, 0, cstr!("PermissionsModel"));
    qml_register_type::<model::Permissions>(cstr!("Webber"), 1, 0, cstr!("Permissions"));

    let mut engine = QmlEngine::new();

    let provider = IconProvider::construct();
    let icon_model = IconModel::construct();
    let icon_model_pinned = unsafe { QObjectPinned::new(&icon_model) };

    unsafe {
        connect(
            icon_model.borrow().get_cpp_object(),
            icon_model
                .borrow()
                .add_image
                .to_cpp_representation(&*icon_model.borrow()),
            |url: &String| provider.borrow_mut().add_image(url.clone()),
        );

        connect(
            provider.borrow().get_cpp_object(),
            provider
                .borrow()
                .image_loaded
                .to_cpp_representation(&*provider.borrow()),
            |url: &String, size: &QSize| icon_model.borrow_mut().image_loaded(url.clone(), *size),
        );
    }

    engine.add_pixmap_provider("webber-icons", provider);
    engine.set_object_property(QString::from("IconModel"), icon_model_pinned);
    std::mem::forget(icon_model);

    engine.load_file("qrc:/qml/Main.qml".into());
    engine.exec();
}

fn init_gettext() {
    let domain = "webber.timsueberkrueb";
    textdomain(domain).expect("Failed to setup gettext domain");

    let app_dir = env::var("APP_DIR").expect("Failed to read the APP_DIR environment variable");

    let mut app_dir_path = PathBuf::from(app_dir);
    if !app_dir_path.is_absolute() {
        app_dir_path = PathBuf::from("/usr");
    }

    let path = app_dir_path.join("share/locale");

    bindtextdomain(domain, path.to_str().unwrap()).expect("Failed to bind gettext domain");
}
