import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.8
import QtQuick.Controls.Suru 2.2

Dialog {
    id: dialog

    property url url
    property bool isFirstLoad: true
    property string screenshotPath

    signal screenshotMade(url source)

    onUrlChanged: webview.url = url;

    onOpened: {
        if (isFirstLoad) {
            screenshotSelector.reset()
            isFirstLoad = false;
        }
        Qt.inputMethod.hide();
    }

    header: Item {
        anchors {
            left: dialog.left
            right: dialog.right
        }

        height: Suru.units.gu(8)

        RowLayout {
            anchors {
                fill: parent
                margins: Suru.units.gu(1)
            }

            MiniBrowserBar {
                id: bar

                Layout.alignment: Qt.AlignVCenter
                Layout.fillWidth: true

                webview: webview
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
            bottom: dialog.bottom
        }

        Item {
            id: browserContainer

            anchors.fill: parent

            MiniBrowserView {
                id: webview
                anchors.fill: parent

                url: url

                screenshotPath: dialog.screenshotPath
                screenshotArea: screenshotSelector.area

                onUrlChanged: bar.setUrl(url)
                onScreenshotMade: {
                    dialog.screenshotMade(source);
                    dialog.close();
                }
            }

            ScreenshotSelector {
                id: screenshotSelector
                anchors.fill: parent
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
                text: i18n.tr("Reset")
                onClicked: screenshotSelector.reset()
            }

            Item { Layout.fillWidth: true }

            Button {
                text: i18n.tr("Screenshot")
                onClicked: webview.makeScreenshot()
            }
        }
    }
}
