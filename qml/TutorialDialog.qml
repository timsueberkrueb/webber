import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.8
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK
import Webber 1.0

AdaptiveDialog {
    id: dialog

    title: i18n.tr("Tutorial")
    iconName: "info"

    content: Item {
        anchors {
            left: parent.left
            top: parent.top
            right: parent.right
        }
        implicitHeight: column.childrenRect.height

        Column {
            id: column

            anchors {
                left: parent.left
                top: parent.top
                right: parent.right
            }

            spacing: Suru.units.gu(1)

            Label {
                text: i18n.tr("Hey, welcome! Webber helps you to create shortcuts for your favorite websites. It's easy!")
                width: parent.width
                wrapMode: Text.WordWrap
            }

            Label {
                text: i18n.tr("Use the webbrowser app to navigate to a website you like to add as a shortcut.")
                width: parent.width
                wrapMode: Text.WordWrap
            }

            UUITK.UbuntuShape {
                width: Suru.units.dp(256)
                height: Suru.units.dp(128)
                backgroundColor: "white"
                sourceFillMode: Image.PreserveAspectCrop
                sourceVerticalAlignment: Image.AlignTop
                source: Image { source: "qrc:/assets/tutorial_browser_share.jpg" }
            }

            Label {
                text: i18n.tr("Use the share option in the menu and select Webber to create the shortcut.")
                width: parent.width
                wrapMode: Text.WordWrap
            }

            Label {
                text: i18n.tr("That's it! Enjoy!")
                width: parent.width
                wrapMode: Text.WordWrap
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
                text: i18n.tr("Browse")
                onClicked: {
                    dialog.close();
                    Qt.openUrlExternally("application:///morph-browser.desktop");
                }
            }

            Item { Layout.fillWidth: true }

            Button {
                text: i18n.tr("Close")
                onClicked: dialog.close()
            }
        }
    }
}
