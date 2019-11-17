// Borrowed from Teleports, MessageListPage.qml

import QtQuick 2.0
import QtQuick.Controls.Suru 2.2

Item {
    id: keyboard

    property var kRect: Qt.inputMethod.keyboardRectangle

    height: kRect.height

    anchors.bottomMargin: Qt.inputMethod.visible ? 0 : -height

    Rectangle {
        color: Suru.backgroundColor
        anchors.fill: parent
    }
}
