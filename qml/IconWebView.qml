import QtQuick 2.0
import QtQuick.Controls 2.0
import QtQuick.Controls.Suru 2.2
import QtWebEngine 1.3
import Lomiri.Components 1.3 as LUITK

Item {
    id: item
    visible: false

    property string screenshotIconPath
    property url source: Qt.resolvedUrl("")
    readonly property bool loading: loader.loading

    function setUrl(url) {
        item.source = Qt.resolvedUrl("");
        loader.targetUrl = url;
    }

    ShaderEffectSource {
        id: grabContainer
        width: Suru.units.dp(256)
        height: Suru.units.dp(256)
        visible: false
        live: false

        Loader {
            id: loader

            anchors.fill: parent

            property url targetUrl
            property bool loading: false

            sourceComponent: emptyComponent

            onTargetUrlChanged: {
                if (targetUrl === "") {
                    loading = false;
                    sourceComponent = emptyComponent;
                } else {
                    loading = true;
                    sourceComponent = emptyComponent;
                    sourceComponent = viewComponent;
                }
            }

            onStatusChanged: {
                if (status === Loader.Ready && sourceComponent === viewComponent) {
                    loading = Qt.binding(function() { return loader.item.loading; });
                    loader.item.url = targetUrl;
                    loader.item.screenshotMade.connect(function(source) {
                        item.source = Qt.resolvedUrl("");
                        item.source = source;
                    });
                }
            }
        }

        Component {
            id: emptyComponent

            Item {}
        }

        Component {
            id: viewComponent

            WebEngineView {
                id: webView

                anchors.fill: parent

                signal screenshotMade(var source)

                onLoadingChanged: {
                    if (loadRequest.status === WebEngineView.LoadSucceededStatus) {
                        grabContainer.scheduleUpdate();
                        grabContainer.grabToImage(function(img) {
                            img.saveToFile(screenshotIconPath);
                            var source = Qt.resolvedUrl("file://" + screenshotIconPath);
                            webView.screenshotMade(source);
                        }, Qt.size(256, 256));
                    }
                }
            }
        }
    }
}
