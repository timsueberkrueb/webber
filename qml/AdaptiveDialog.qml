import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.8
import QtQuick.Controls.Suru 2.2
import Lomiri.Components 1.3 as LUITK

Dialog {
    id: dialog

    property string iconName
    property string text
    property Item content

    parent: App.dialogContainer

    x: (parent.width - width) / 2
    y: (parent.height - height) / 2

    onClosed: Qt.inputMethod.hide()

    width: parent.width - Suru.units.gu(4)
    height: Math.min(implicitHeight, parent.height - Suru.units.gu(4))

    modal: true
    closePolicy: Dialog.NoAutoClose

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

            LUITK.Icon {
                name: dialog.iconName
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
            contentHeight: container.implicitHeight
            implicitHeight: contentHeight
            interactive: contentHeight > height

            Item {
                id: container

                anchors {
                    left: parent.left
                    top: parent.top
                    right: parent.right
                }

                implicitHeight: childrenRect.height
                children: dialog.content
            }
        }
    }
}
