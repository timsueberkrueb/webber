#[macro_use]
extern crate cstr;
#[macro_use]
extern crate cpp;
#[macro_use]
extern crate qmetaobject;

use qmetaobject::*;

mod click;
mod core;
mod model;
mod qrc;

fn main() {
    unsafe {
        cpp! { {
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
