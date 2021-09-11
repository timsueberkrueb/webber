import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.8
import QtQuick.Controls.Suru 2.2
import QtGraphicalEffects 1.0
import Ubuntu.Components 1.3 as UUITK
import Webber 1.0
import "."

AdaptiveDialog {
    id: dialog

    title: i18n.tr("About")
    iconName: "info"

    onOpened: webberAnim.startDelayed(250)
    onAboutToHide: webberAnim.stop()

    content: Rectangle {
        anchors {
            left: parent.left
            top: parent.top
            right: parent.right
        }
        implicitHeight: column.childrenRect.height

        Column {
            id: column

            anchors {
                left: parent.left
                top: parent.top
                right: parent.right
            }

            spacing: Suru.units.gu(1)

            Label {
                text: "Webber"
                horizontalAlignment: Text.AlignHCenter
                width: parent.width
                wrapMode: Text.WordWrap
                font: Suru.units.fontHeadingOne
            }

            Item {
                width: parent.width
                height: Math.min(parent.width, Suru.units.dp(128))

                Rectangle {
                    anchors.centerIn: parent

                    color: App.themeColor
                    width: Math.min(parent.width, Suru.units.dp(128))
                    height: Math.min(parent.width, Suru.units.dp(128))
                    clip: true

                    Rectangle {
                        id: foreground
                        anchors.fill: parent
                        color: "white"
                        visible: false
                    }

                    WebberAnimation {
                        id: webberAnim
                        anchors.centerIn: parent
                        width: cutoutCircle.width
                        height: cutoutCircle.height
                    }

                    Item {
                        id: cutout
                        anchors {
                            fill: parent
                            margins: Suru.units.gu(1)
                        }
                        visible: false

                        Rectangle {
                            id: cutoutCircle
                            anchors.centerIn: parent
                            radius: width * 0.5
                            width: Math.min(parent.width, parent.height)
                            height: width
                        }
                    }

                    OpacityMask {
                        id: opacityMask
                        anchors.fill: parent
                        source: foreground
                        maskSource: cutout
                        invert: true
                    }
                }
            }

            Label {
                text: i18n.tr("Webapp shortcut creator for Ubuntu touch")
                horizontalAlignment: Text.AlignHCenter
                width: parent.width
                wrapMode: Text.WordWrap
                font: Suru.units.fontHeadingThree
            }

            Item { height: Suru.units.gu(1) }

            Column {
                width: parent.width
                spacing: Suru.units.gu(1)

                Button {
                    anchors.horizontalCenter: parent.horizontalCenter
                    text: "❤️ %1".arg(i18n.tr("Rate"))
                    onClicked: Qt.openUrlExternally("openstore://webber.timsueberkrueb")
                }

                Button {
                    anchors.horizontalCenter: parent.horizontalCenter
                    text: "✨ %1".arg(i18n.tr("Contribute"))
                    onClicked: Qt.openUrlExternally("https://github.com/timsueberkrueb/webber")
                }

                Button {
                    anchors.horizontalCenter: parent.horizontalCenter
                    text: "🐛 %1".arg(i18n.tr("Report bug"))
                    onClicked: Qt.openUrlExternally("https://github.com/timsueberkrueb/webber/issues/new")
                }

                Button {
                    anchors.horizontalCenter: parent.horizontalCenter
                    text: i18n.tr("Close")
                    onClicked: dialog.close()
                }
            }
        }
    }
}
