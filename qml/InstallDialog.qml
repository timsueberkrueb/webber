import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Ubuntu.Content 1.3
import "."

Dialog {
    id: dialog

    property url url: Qt.resolvedUrl("file:///home/phablet/.cache/webber.timsueberkrueb/click-build/shortcut.click")

    x: (parent.width - width) / 2
    y: (parent.height - height) / 2

    title: i18n.tr('Install shortcut')

    contentItem: Item {
        implicitWidth: dialog.contentWidth
        implicitHeight: dialog.contentHeight

        ContentPeerPicker {
            id: picker

            property var activeTransfer

            anchors.fill: parent
            showTitle: false
            contentType: ContentType.All
            handler: ContentHandler.Destination

            onPeerSelected: {
                picker.activeTransfer = peer.request()
                picker.activeTransfer.stateChanged.connect(function() {
                    if (picker.activeTransfer.state === ContentTransfer.InProgress) {
                        picker.activeTransfer.items = [ resultComponent.createObject(parent, {"url": url}) ];
                        picker.activeTransfer.state = ContentTransfer.Charged;
                        dialog.close()
                    }
                })
            }

            Component {
                id: resultComponent

                ContentItem {}
            }
        }

        ContentTransferHint {
            id: transferHint
            anchors.fill: parent
            activeTransfer: picker.activeTransfer
        }
    }

    standardButtons: Dialog.NoButton
    modal: true
    closePolicy: Dialog.NoAutoClose
}
