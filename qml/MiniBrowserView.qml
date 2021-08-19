import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.0
import QtQuick.Controls.Suru 2.2
import QtWebEngine 1.3
import Ubuntu.Components 1.3 as UUITK

WebEngineView {
    id: webview

    property string screenshotPath
    property rect screenshotArea

    zoomFactor: (units.gridUnit / 8)

    signal screenshotMade(url source)

    function makeScreenshot() {
        grabArea.scheduleUpdate();
        grabArea.grabToImage(function(img) {
            img.saveToFile(screenshotPath);
            var source = Qt.resolvedUrl("file://" + screenshotPath);
            webview.screenshotMade(source);
        }, Qt.size(grabArea.width, grabArea.height));
    }

    onUrlChanged: {
        zoomFactor = (units.gridUnit / 8);
    }

    onLoadingChanged: {
        if (loadRequest.status === WebEngineView.LoadSucceededStatus) {
            zoomFactor = (units.gridUnit / 8);
        }
    }

    ShaderEffectSource {
        id: grabArea

        visible: false
        live: false

        sourceItem: webview
        sourceRect: screenshotArea

        width: screenshotArea.width
        height: screenshotArea.height
    }
}
