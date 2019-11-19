import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.0
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK
import "."

Page {
    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            spacing: units.gu(1)

            UUITK.UbuntuShape {
                implicitWidth: Suru.units.gu(4)
                implicitHeight: Suru.units.gu(4)

                source: Image {
                    source: Qt.resolvedUrl("qrc:///assets/icon.svg")
                    sourceSize.width: Suru.units.gu(4)
                    sourceSize.height: Suru.units.gu(4)
                }
            }

            Label {
                text: "Webber"
                font.pixelSize: units.dp(16)
            }

            Item { Layout.fillWidth: true }

            Button {
                text: "Add"
                onClicked: App.stackView.push(addPage);
            }
        }
    }

    Item {
        anchors {
            fill: parent
            margins: 16
        }

        Item {
            anchors.centerIn: parent
            width: parent.width - units.gu(4)
            height: column.height

            Column {
                id: column

                spacing: Suru.units.gu(2)
                width: parent.width
                height: childrenRect.height

                Row {
                    anchors.horizontalCenter: parent.horizontalCenter
                    spacing: Suru.units.gu(2)

                    UUITK.Icon {
                        width: Suru.units.gu(10)
                        height: Suru.units.gu(10)
                        name: "webbrowser-app-symbolic"
                    }

                    UUITK.Icon {
                        width: Suru.units.gu(10)
                        height: Suru.units.gu(10)
                        name: "share"
                    }
                }

                Label {
                    width: parent.width
                    text: "Use the webbrowser app to navigate to a website you like to add as a shortcut. " +
                          "Use the share option in the menu and select Webber to create the shortcut."
                    wrapMode: Text.WordWrap
                    horizontalAlignment: Qt.AlignHCenter
                }

                Button {
                    anchors.horizontalCenter: parent.horizontalCenter
                    text: "Browse"
                    onClicked: Qt.openUrlExternally("application:///morph-browser.desktop")
                }
            }
        }
    }
}
