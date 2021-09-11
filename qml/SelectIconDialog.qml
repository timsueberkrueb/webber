import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.8
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK
import Webber 1.0

AdaptiveDialog {
    id: dialog

    property string selectedIconUrl

    function resetSelected() {
        iconsView.currentIndex = 0;
        selectedIconUrl = IconModel.get(iconsView.currentIndex);
    }

    title: i18n.tr("Select icon")
    iconName: "stock_website"

    content: Item {
        id: content

        anchors {
            left: parent.left
            top: parent.top
            right: parent.right
        }

        readonly property bool isEmpty: iconsView.count == 0

        implicitHeight: content.isEmpty ? noIconsLabel.implicitHeight : iconsView.contentHeight

        Label {
            id: noIconsLabel
            anchors {
                left: parent.left
                top: parent.top
                right: parent.right
            }
            visible: content.isEmpty
            wrapMode: Text.WordWrap
            text: i18n.tr("No icons found in the website metadata, sorry! You could try to use a screenshot or custom icon instead.")
        }

        GridView {
            id: iconsView

            anchors {
                left: parent.left
                top: parent.top
                right: parent.right
            }

            onCurrentIndexChanged: {
                selectedIconUrl = IconModel.get(iconsView.currentIndex);
            }

            model: IconModel
            visible: !content.isEmpty
            interactive: false

            cellWidth: Suru.units.gu(12)
            cellHeight: Suru.units.gu(16)

            delegate: Component {
                Rectangle {
                    width: Suru.units.gu(10)
                    height: Suru.units.gu(14)
                    color: isSelected ? "#19B6EE" : "transparent"
                    radius: Suru.units.dp(4)

                    readonly property bool isSelected: iconsView.currentIndex === index
                    readonly property bool isSVG: model.url.toLowerCase().endsWith(".svg")

                    UUITK.UbuntuShape {
                        id: ubuntuShape

                        anchors {
                            left: parent.left
                            top: parent.top
                            right: parent.right
                            margins: Suru.units.gu(1)
                        }

                        backgroundColor: "white"
                        width: Suru.units.gu(8)
                        height: Suru.units.gu(8)
                        aspect: isSelected ? UUITK.UbuntuShape.Flat : UUITK.UbuntuShape.Inset

                        source: Image {
                            id: iconImage
                            source: "image://webber-icons/%1".arg(model.url)
                            sourceSize.width: Suru.units.gu(8)
                            sourceSize.height: Suru.units.gu(8)
                            cache: false
                        }

                        BusyIndicator {
                            id: busyIndicator
                            anchors.centerIn: parent
                            running: iconImage.status == Image.Loading
                        }
                    }

                    Rectangle {
                        anchors {
                            top: ubuntuShape.bottom
                            horizontalCenter: parent.horizontalCenter
                            topMargin: Suru.units.gu(1)
                        }

                        color: isSelected ? "transparent" : (isSVG ? "#3B3B3B" : "#666666")
                        radius: Suru.units.dp(4)
                        width: label.width + Suru.units.gu(1)
                        height: label.height + Suru.units.dp(4)

                        Label {
                            id: label

                            anchors.centerIn: parent

                            text: isSVG ? "SVG" : model.sourceWidth.toString() + "×" + model.sourceHeight.toString()
                            color: "white"
                            horizontalAlignment: Qt.AlignHCenter
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            iconsView.currentIndex = index;
                            iconImage.source = "";
                            iconImage.source = "image://webber-icons/%1".arg(model.url);
                        }
                    }
                }
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

            Item { Layout.fillWidth: true }

            Button {
                text: i18n.tr("Close")
                onClicked: dialog.close()
            }
        }
    }

    Connections {
        target: IconModel
        onModelChanged: dialog.resetSelected()
    }
}
