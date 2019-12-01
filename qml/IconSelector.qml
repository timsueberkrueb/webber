import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK

RowLayout {
    id: iconSelector

    readonly property bool useScreenshotIcon: iconScreenshot.checked
    property string screenshotIconPath
    property url defaultIconUrl: Qt.resolvedUrl("")

    function setUrl(url) {
        iconWebView.setUrl(url);
    }

    function loadDefaults() {
        iconWebView.setUrl(Qt.resolvedUrl(""));
        iconDefault.checked = true;
    }

    spacing: units.gu(1)

    ButtonGroup {
        buttons: [
            iconDefault.radioButton,
            iconScreenshot.radioButton
        ]
    }

    IconSelectItem {
        id: iconDefault
        text: "Default"
        helpText: "Icon specified in website meta data"
        source: defaultIconUrl
        checked: true
    }

    IconSelectItem {
        id: iconScreenshot
        text: "Screenshot"
        source: iconWebView.source
        loading: iconWebView.loading
    }

    IconWebView {
        id: iconWebView
        screenshotIconPath: iconSelector.screenshotIconPath
    }
}
