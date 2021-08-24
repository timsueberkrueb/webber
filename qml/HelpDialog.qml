import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.8
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK

Dialog {
    id: dialog

    property url url
    property string text

    header: Item {
        anchors {
            left: dialog.left
            right: dialog.right
        }

        implicitHeight: Suru.units.gu(8)

        RowLayout {
            anchors {
                fill: parent
                margins: Suru.units.gu(1)
            }

            spacing: Suru.units.gu(1)

            UUITK.Icon {
                name: "info"
                width: Suru.units.gu(4)
                height: Suru.units.gu(4)
            }

            Label {
                Layout.fillWidth: true
                text: dialog.title
                font: Suru.units.fontHeadingThree
            }

            IconButton {
                Layout.alignment: Qt.AlignVCenter
                iconName: "close"
                onClicked: dialog.close()
            }
        }
    }

    standardButtons: Dialog.NoButton

    contentItem: Item {
        anchors {
            top: dialog.header.bottom
            left: dialog.left
            right: dialog.right
        }

        implicitHeight: flickable.implicitHeight
        height: Math.min(implicitHeight, parent.height)

        clip: true

        Flickable {
            id: flickable
            anchors.fill: parent
            contentHeight: label.height
            implicitHeight: contentHeight
            interactive: contentHeight > height

            Label {
                id: label

                anchors {
                    left: parent.left
                    top: parent.top
                    right: parent. right
                }

                text: dialog.text
                wrapMode: Text.WordWrap
                onLinkActivated: Qt.openUrlExternally(link)
            }
        }
    }

    footer: Item {
        implicitHeight: Suru.units.gu(8)

        RowLayout {
            anchors {
                fill: parent
                leftMargin: units.gu(2)
                rightMargin: units.gu(2)
            }

            Button {
                text: i18n.tr("Learn more")
                onClicked: Qt.openUrlExternally(dialog.url)
            }

            Item { Layout.fillWidth: true }

            Button {
                text: i18n.tr("Close")
                onClicked: dialog.close()
            }
        }
    }
}
