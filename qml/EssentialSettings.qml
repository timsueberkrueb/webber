import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK

Item {
    id: essentialSettings

    readonly property bool useScreenshotIcon: iconSelector.useScreenshotIcon
    readonly property bool useCustomIcon: iconSelector.useCustomIcon
    property url iconUrl
    property string url
    property var scraper
    property var appModel
    property alias name: nameField.text
    property url customIconSource

    function loadDefaults() {
        nameField.text = "";
        iconUrl = "";
        iconSelector.loadDefaults();
    }

    signal customIconRequested()
    signal refresh()

    onRefresh: iconSelector.setUrl(url)

    implicitHeight: column.childrenRect.height

    ColumnLayout {
        id: column
        width: parent.width

        spacing: Suru.units.gu(1)

        Label {
            text: "Properties"
            font.bold: true
        }

        Rectangle {
            id: failLoadBox

            visible: scraper.errorString !== "" && url !== ""

            Layout.fillWidth: true

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
                        text: "Refresh"
                        onClicked: {
                            d.refresh();
                        }
                    }
                }
            }
        }

        GridLayout {
            Layout.fillWidth: true

            columns: 2
            columnSpacing: Suru.units.gu(1)
            rowSpacing: Suru.units.gu(1)

            Label {
                text: "Name"
            }

            TextField {
                id: nameField
                Layout.fillWidth: true
                placeholderText: "Web app name"
            }
        }

        Label {
            text: "Icon"
            font.bold: true
        }

        IconSelector {
            id: iconSelector
            Layout.fillWidth: true
            defaultIconUrl: iconUrl
            screenshotIconPath: appModel.screenshotIconPath
            customIconSource: essentialSettings.customIconSource
            onCustomIconRequested: essentialSettings.customIconRequested()
        }
    }
}
