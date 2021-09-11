pragma Singleton

import QtQuick 2.0

QtObject {
    readonly property color themeColor: "#80a3fa"
    property var dialogContainer: null
    property var stackView: null
    property var helpDialog: null

    function showHelp(title, text, url) {
        helpDialog.title = title;
        helpDialog.text = text;
        helpDialog.url = url;
        helpDialog.open();
    }

    function pop() {
        if (stackView) {
            stackView.pop();
        } else {
            console.error("StackView not set");
        }
    }
}
