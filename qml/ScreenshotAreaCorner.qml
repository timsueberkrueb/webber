import QtQuick 2.12
import QtQuick.Controls.Suru 2.2
import QtQuick.Controls 2.0

Item {
    id: areaCorner

    property string cornerID
    property point cornerPoint

    x: cornerPoint.x - width/2
    y: cornerPoint.y - height/2

    width: Suru.units.dp(48)
    height: Suru.units.dp(48)

    MouseArea {
        id: mouseArea

        anchors.fill: parent

        drag.target: circle
        drag.threshold: 0

        onReleased: circle.Drag.drop()

        Rectangle {
            id: fixedCircle

            anchors {
                horizontalCenter: parent.horizontalCenter
                verticalCenter: parent.verticalCenter
            }

            width: Suru.units.dp(16)
            height: Suru.units.dp(16)

            radius: width/2
            color: Suru.highlightColor
        }

        Rectangle {
            id: circle

            anchors {
                horizontalCenter: parent.horizontalCenter
                verticalCenter: parent.verticalCenter
            }

            property string cornerID: areaCorner.cornerID

            signal dropped(var drop);

            function dist(obj1, obj2) {
                return Math.sqrt(Math.pow(obj1.x - obj2.x, 2) + Math.pow(obj1.y - obj2.y, 2));
            }

            Drag.keys: ["resize"]
            Drag.active: mouseArea.drag.active
            Drag.hotSpot: Qt.point(width/2, height/2)

            width: Suru.units.dp(32)
            height: Suru.units.dp(32)

            radius: width/2
            color: Suru.highlightColor
            opacity: Drag.active ? 0.5 - Math.min(0.5, Math.max(0, dist(circle, fixedCircle)/Suru.units.dp(64))) : 0

            states: State {
                when: mouseArea.drag.active
                ParentChange { target: circle; parent: areaCorner }
                AnchorChanges { target: circle; anchors.verticalCenter: undefined; anchors.horizontalCenter: undefined }
            }
        }
    }
}
