import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Lomiri.Content 1.3
import "."

Dialog {
    id: dialog

    property string clickPath: ""
    property url url: Qt.resolvedUrl("file://" + clickPath)

    title: i18n.tr("Exporting ...")

    onOpened: exportToOpenStore()

    function exportToOpenStore() {
        var app = "openstore.openstore-team_";
        var peer = null;
        for (var i = 0; i < model.peers.length; ++i) {
            var p = model.peers[i];
            if (p.appId.indexOf(app) === 0) {
                peer = p
                break;
            }
        }
        if (peer !== null) {
            peer.contentType = ContentType.All;
            peer.selectionType = ContentTransfer.Single;
            model.activeTransfer = peer.request();
            model.activeTransfer.stateChanged.connect(function() {
                if (model.activeTransfer.state === ContentTransfer.InProgress) {
                    model.activeTransfer.items = [ resultComponent.createObject(parent, {"url": url}) ];
                    model.activeTransfer.state = ContentTransfer.Charged;
                    dialog.close();
                }
            });
        } else {
            console.error("Failed to select peer");
        }
    }

    contentItem: Item {
        implicitWidth: Suru.units.dp(128)
        implicitHeight: Suru.units.dp(128)

        ContentPeerModel {
            id: model

            property var activeTransfer: null

            contentType: ContentType.All
            handler: ContentHandler.Destination
        }

        BusyIndicator {
            anchors.centerIn: parent
            running: model.activeTransfer !== null
        }

        Component {
            id: resultComponent

            ContentItem {}
        }
    }

    standardButtons: Dialog.NoButton
    modal: true
    closePolicy: Dialog.NoAutoClose
}
