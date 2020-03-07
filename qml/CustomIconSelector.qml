import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK
import Ubuntu.Content 1.3

Dialog {
    id: dialog

    title: i18n.tr("Import custom icon")

    property url source
    property var activeTransfer

    contentItem: Item {
        ContentPeerPicker {
            id: peerPicker

            anchors.fill: parent

            contentType: ContentType.Pictures
            handler: ContentHandler.Source
            showTitle: false

            onPeerSelected: {
                peer.selectionType = ContentTransfer.Single
                activeTransfer = peer.request()
            }

            onCancelPressed: {
                PopupUtils.close(root.importDialog)
            }
        }
    }

    Connections {
        target: activeTransfer
        onStateChanged: {
            if (activeTransfer.state === ContentTransfer.Charged) {
                if (activeTransfer.items.length > 0) {
                    dialog.source = activeTransfer.items[0].url;
                }
                dialog.close();
            }
        }
    }

    standardButtons: Dialog.Cancel
    modal: true
    closePolicy: Dialog.NoAutoClose
}
