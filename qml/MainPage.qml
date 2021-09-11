import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import QtGraphicalEffects 1.0
import Ubuntu.Components 1.3 as UUITK
import Webber 1.0
import "."

Page {
    id: mainPage

    header: ToolBar {
        height: Suru.units.gu(6)

        background: Rectangle { color: App.themeColor }

        RowLayout {
            anchors.fill: parent

            spacing: units.gu(1)

            Image {
                source: Qt.resolvedUrl("qrc:///assets/webber_spider.svg")
                sourceSize.width: Suru.units.gu(4)
                sourceSize.height: Suru.units.gu(4)
            }

            Label {
                text: "Webber"
                color: "white"
                font: Suru.units.fontHeadingTwo
            }

            Item { Layout.fillWidth: true }

            Button {
                text: i18n.tr("Add")
                onClicked: {
                    App.stackView.push(addPage);
                    addPage.setUrl("");
                }
            }
        }
    }

    StaticMainPageContent {
        anchors.fill: parent
    }

    RoundButton {
        anchors {
            left: parent.left
            bottom: parent.bottom
            margins: Suru.units.dp(32)
        }

        contentItem: UUITK.Icon { name: "help" }
        onClicked: tutorialDialog.open()
    }

    RoundButton {
        anchors {
            right: parent.right
            bottom: parent.bottom
            margins: Suru.units.dp(32)
        }

        contentItem: UUITK.Icon { name: "info" }
        onClicked: aboutDialog.open()
    }

    TutorialDialog {
        id: tutorialDialog
    }

    AboutDialog {
        id: aboutDialog
    }
}
