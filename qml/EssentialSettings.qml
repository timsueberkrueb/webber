import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2

Item {
    property alias iconUrl: iconImage.source
    property string url
    property var scraper
    property alias name: nameField.text

    function loadDefaults() {
        nameField.text = "";
        iconImage.source = "";
    }

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

            Label {
                text: "Icon"
            }

            Item {
                implicitWidth: Suru.units.gu(8)
                implicitHeight: Suru.units.gu(8)

                Image {
                    id: iconImage

                    anchors.fill: parent
                    sourceSize.width: Suru.units.gu(8)
                    sourceSize.height: Suru.units.gu(8)

                    BusyIndicator {
                        anchors.centerIn: parent
                        running: iconImage.status == Image.Loading
                    }
                }

            }
        }
    }
}
