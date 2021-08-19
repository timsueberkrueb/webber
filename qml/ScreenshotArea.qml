import QtQuick 2.0
import QtQuick.Controls.Suru 2.2
import QtQuick.Controls 2.0

Rectangle {
    id: shotRectangle

    property real startX: 0
    property real startY: 0

    property real dragX
    property real dragY
    property string dragID

    property point pivot

    property real delta: Math.min(Math.abs(dragX - pivot.x), Math.abs(dragY - pivot.y))
    property real deltaX: startX !== pivot.x ? dragSign().x * delta : 0
    property real deltaY: startY !== pivot.y ? dragSign().y * delta : 0

    readonly property point topLeft: Qt.point(x, y)
    readonly property point topRight: Qt.point(x + width, y)
    readonly property point bottomRight: Qt.point(x + width, y + height)
    readonly property point bottomLeft: Qt.point(x, y + height)

    readonly property bool isMoving: Drag.active

    function dragSignMatches() {
        var expected = d.dragSign(dragID);
        var actual = Qt.point(d.sign(dragX - pivot.x), d.sign(dragY - pivot.y));
        return actual === expected;
    }

    function dragSign() {
        return dragSignMatches() ? d.dragSign(dragID) : Qt.point(0, 0);
    }

    readonly property QtObject cornerID: QtObject {
        readonly property string topLeft: "topLeft"
        readonly property string topRight: "topRight"
        readonly property string bottomRight: "bottomRight"
        readonly property string bottomLeft: "bottomLeft"
    }

    function cornerPoint(corner) {
        switch (corner) {
            case cornerID.topLeft:
                return shotArea.topLeft;
            case cornerID.topRight:
                return shotArea.topRight;
            case cornerID.bottomRight:
                return shotArea.bottomRight;
            case cornerID.bottomLeft:
                return shotArea.bottomLeft;
        }
    }

    function pivotFor(corner) {
        switch (corner) {
            case cornerID.topLeft:
                return cornerID.bottomRight;
            case cornerID.topRight:
                return cornerID.bottomLeft;
            case cornerID.bottomRight:
                return cornerID.topLeft;
            case cornerID.bottomLeft:
                return cornerID.topRight;
        }
    }

    Drag.keys: ["move"]
    Drag.active: mouseArea.drag.active
    Drag.hotSpot: Qt.point(width/2, height/2)

    implicitWidth: Suru.units.dp(64)
    implicitHeight: Suru.units.dp(64)

    color: "transparent"
    border {
        color: Suru.highlightColor
        width: Suru.units.dp(2)
    }

    state: dragID !== "" && delta > Suru.units.dp(64) && dragSignMatches() ? "resizing" : "static"
    states: [
        State {
            name: "static"
            PropertyChanges {
                target: shotRectangle
                explicit: true
                restoreEntryValues: false
                x: shotRectangle.x
                y: shotRectangle.y
                width: Math.max(delta, Suru.units.dp(64))
                height: Math.max(delta, Suru.units.dp(64))
            }
        },
        State {
            name: "resizing"
            PropertyChanges {
                target: shotRectangle
                restoreEntryValues: false
                x: pivot.x + deltaX
                y: pivot.y + deltaY
                width: Math.max(delta, Suru.units.dp(64))
                height: Math.max(delta, Suru.units.dp(64))
            }
        }
    ]

    MouseArea {
        id: mouseArea

        anchors.fill: parent

        drag.target: shotRectangle
        drag.threshold: 0

        onReleased: shotRectangle.Drag.drop()
    }

    QtObject {
        id: d

        function dragSign(corner) {
            switch (corner) {
                case cornerID.topLeft:
                    return Qt.point(-1, -1);
                case cornerID.topRight:
                    return Qt.point(1, -1);
                case cornerID.bottomRight:
                    return Qt.point(1, 1);
                case cornerID.bottomLeft:
                    return Qt.point(-1, 1);
            }
        }

        function sign(x) {
            return x >= 0 ? 1 : -1;
        }
    }
}
