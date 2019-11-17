import QtQuick 2.0
import Ubuntu.Content 1.3

QtObject {
    id: contentImport

    signal urlRequested(url url)

    property var d: QtObject {
        id: d

        function startImport( transfer ) {
            if (transfer.contentType === ContentType.Links) {
                if (transfer.items.length >= 1) {
                    contentImport.urlRequested(transfer.items[0].url);
                }
            }
        }

        property var conn: Connections {
            target: ContentHub
            onShareRequested: d.startImport(transfer)
        }
    }
}
