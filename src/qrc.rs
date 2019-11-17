qrc!(qml_resources,

    "/" {
        "qml/AddPage.qml",
        "qml/App.qml",
        "qml/ContentImport.qml",
        "qml/IconButton.qml",
        "qml/InstallDialog.qml",
        "qml/KeyboardPlaceholder.qml",
        "qml/Main.qml",
        "qml/qmldir",
    },
);

pub fn load() {
    qml_resources();
}
