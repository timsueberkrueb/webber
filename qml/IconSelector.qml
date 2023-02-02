import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Lomiri.Components 1.3 as LUITK

RowLayout {
    id: iconSelector

    readonly property bool useScreenshotIcon: iconScreenshot.checked
    readonly property bool useCustomIcon: iconCustom.checked
    property string defaultIconUrl
    property string screenshotIconPath
    property url customIconSource

    signal selectIconRequested()
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
        sourcePrefix: "image://webber-icons/"
        checked: true
        onIconClicked: iconSelector.selectIconRequested()
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
