import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Lomiri.Components 1.3 as LUITK

Item {
    id: essentialSettings

    readonly property bool useScreenshotIcon: iconSelector.useScreenshotIcon
    readonly property bool useCustomIcon: iconSelector.useCustomIcon
    property string iconUrl
    property string url
    property var scraper
    property var appModel
    property alias name: nameField.text
    property url customIconSource

    function loadDefaults() {
        nameField.text = "";
        iconSelector.loadDefaults();
    }

    signal selectIconRequested()
    signal customIconRequested()
    signal screenshotRequested()
    signal screenshotMade()
    signal refresh()

    onScreenshotMade: iconSelector.screenshotMade()

    implicitHeight: column.childrenRect.height

    Column {
        id: column
        width: parent.width

        spacing: Suru.units.gu(1)

        Label {
            text: i18n.tr("Properties")
            font.bold: true
        }

        Rectangle {
            id: failLoadBox

            visible: scraper.errorString !== "" && url !== ""

            width: parent.width

            implicitHeight: childrenRect.height + Suru.units.gu(2)

            radius: Suru.units.dp(4)
            border.width: Suru.units.dp(1)
            border.color: Suru.neutralColor

            Column {
                width: parent.width - Suru.units.gu(2)
                x: Suru.units.gu(1)
                y: Suru.units.gu(1)
                spacing: Suru.units.gu(1)

                Label {
                    width: parent.width
                    text: scraper.errorString
                    wrapMode: Text.WordWrap
                    color: Suru.color(Suru.Red)
                }

                RowLayout {
                    width: parent.width

                    Item {
                        Layout.fillWidth: true
                    }

                    Button {
                        text: i18n.tr("Refresh")
                        onClicked: {
                            d.refresh();
                        }
                    }
                }
            }
        }

        GridLayout {
            width: parent.width

            columns: 2
            columnSpacing: Suru.units.gu(1)
            rowSpacing: Suru.units.gu(1)

            Label {
                text: i18n.tr("Name")
            }

            LUITK.TextField {
                id: nameField
                Layout.fillWidth: true
                placeholderText: i18n.tr("Web app name")
            }
        }

        Label {
            text: i18n.tr("Icon")
            font.bold: true
        }

        Flickable {
            width: parent.width
            height: iconSelContent.height
            contentWidth: iconSelContent.width
            interactive: width < contentWidth

            Item {
                id: iconSelContent

                height: iconSelector.implicitHeight
                width: iconSelector.width

                IconSelector {
                    id: iconSelector
                    defaultIconUrl: iconUrl
                    screenshotIconPath: appModel.screenshotIconPath
                    customIconSource: essentialSettings.customIconSource
                    onSelectIconRequested: essentialSettings.selectIconRequested()
                    onCustomIconRequested: essentialSettings.customIconRequested()
                    onScreenshotRequested: essentialSettings.screenshotRequested()
                }
            }
        }
    }
}
