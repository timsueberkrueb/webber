import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK

RowLayout {
    id: iconSelector

    readonly property bool useScreenshotIcon: iconScreenshot.checked
    readonly property bool useCustomIcon: iconCustom.checked
    property url defaultIconUrl: Qt.resolvedUrl("")
    property string screenshotIconPath
    property url customIconSource

    signal screenshotRequested()
    signal customIconRequested()
    signal screenshotMade()

    function loadDefaults() {
        iconScreenshot.reload();
        iconDefault.checked = true;
    }

    onScreenshotMade: iconScreenshot.reload()

    spacing: units.gu(1)

    ButtonGroup {
        buttons: [
            iconDefault.radioButton,
            iconScreenshot.radioButton,
            iconCustom.radioButton
        ]
    }

    IconSelectItem {
        id: iconDefault
        /// i18n: Label below the icon selection
        text: i18n.tr("Default")
        helpText: i18n.tr("Icon specified in website meta data")
        placeholderIconName: "stock_website"
        source: defaultIconUrl
        checked: true
    }

    IconSelectItem {
        id: iconScreenshot

        function reload() {
            var newSource = screenshotIconPath;
            source = "";
            source = Qt.resolvedUrl("file://" + newSource);
        }

        /// i18n: Label below the icon selection
        text: i18n.tr("Screenshot")
        placeholderIconName: "camera-grid"
        onIconClicked: iconSelector.screenshotRequested()
    }

    IconSelectItem {
        id: iconCustom
        /// i18n: Label below the icon selection
        text: i18n.tr("Custom")
        placeholderIconName: "insert-image"
        source: iconSelector.customIconSource
        onIconClicked: iconSelector.customIconRequested()
    }
}
