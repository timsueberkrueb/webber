import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.8
import QtQuick.Controls.Suru 2.2
import Lomiri.Components 1.3 as LUITK

AdaptiveDialog {
    id: dialog

    property url url
    property string text

    iconName: "info"

    content: Label {
        anchors {
            left: parent.left
            top: parent.top
            right: parent.right
        }
        text: dialog.text
        wrapMode: Text.WordWrap
        onLinkActivated: Qt.openUrlExternally(link)
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
