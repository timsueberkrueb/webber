#![feature(generic_associated_types)]

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
mod model;
mod pwa;
mod qrc;
mod resolvable;
mod scraper;
mod serde_utils;

fn main() {
    init_gettext();

    unsafe {
        cpp! {{
            #include <QtCore/QCoreApplication>
            #include <QtCore/QString>
            #include <QtWebEngine/QtWebEngine>
        }}
        cpp! {[]{
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

    unsafe {
        cpp! {[]{
            QtWebEngine::initialize();
        }}
    }

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
