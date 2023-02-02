import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.0
import QtQuick.Controls.Suru 2.2
import QtWebEngine 1.3
import Lomiri.Components 1.3 as LUITK

Item {
    id: item

    property var webview

    function setUrl(url) {
        urlField.text = url;
    }

    implicitHeight: units.gu(6)

    RowLayout {
        anchors.fill: parent

        IconButton {
            iconName: "go-previous"
            enabled: webview.canGoBack
            onClicked: webview.goBack()
        }

        IconButton {
            id: forwardButton
            iconName: "go-next"
            opacity: webview.canGoForward && !forwardAnimation.running ? 1 : 0
            implicitWidth: webview.canGoForward ? Suru.units.gu(4) : 0
            enabled: webview.canGoForward
            onClicked: webview.goForward()

            Behavior on implicitWidth {
                NumberAnimation {
                    id: forwardAnimation
                    duration: 100
                    easing.type: Easing.InOutQuad
                }
            }

            Behavior on opacity {
                NumberAnimation {
                    duration: 100
                    easing.type: Easing.InOutQuad
                }
            }
        }

        LUITK.TextField {
            id: urlField

            property string previousText: ""
            signal editingFinished()

            Layout.fillWidth: true

            /// i18n: %1 is a placeholder for an example url. Do not change the %1!
            placeholderText: i18n.tr("Url (e.g. %1)").arg("https://example.com")
            inputMethodHints: Qt.ImhUrlCharactersOnly
            onAccepted: editingFinished()
            onActiveFocusChanged: if (!activeFocus) editingFinished()
            onEditingFinished: {
                if (displayText !== previousText) {
                    // Prepend https by default
                    if (displayText.trim() !== "" && displayText.trim().indexOf("http") !== 0) {
                        text = "https://" + displayText.trim();
                    }
                    previousText = displayText;
                }
                webview.url = text
            }
        }
    }
}
