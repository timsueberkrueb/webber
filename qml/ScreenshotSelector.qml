import QtQuick 2.0
import QtQuick.Controls.Suru 2.2
import QtQuick.Controls 2.0
import QtGraphicalEffects 1.0

Item {
    id: screenshotSelector

    readonly property rect area: Qt.rect(shotArea.x, shotArea.y, shotArea.width, shotArea.height)

    function reset() {
        shotArea.height = units.dp(128);
        shotArea.width = units.dp(128);
        shotArea.x = (width - shotArea.width)/2;
        shotArea.y = (height - shotArea.height)/2;
    }

    clip: true

    Rectangle {
        id: background
        color: "black"
        height: screenshotSelector.height
        width: screenshotSelector.width
        visible: false
    }

    Item {
        id: cutout
        anchors.fill: parent
        visible: false

        Rectangle {
            x: shotArea.x
            y: shotArea.y
            width: shotArea.width
            height: shotArea.height
        }
    }

    OpacityMask {
        anchors.fill: parent
        opacity: shotArea.isMoving ? 0.5 : 0.3
        source: background
        maskSource: cutout
        invert: true

        Behavior on opacity {
            NumberAnimation {
                duration: 500
            }
        }
    }

    DropArea {
        id: dropArea

        anchors.fill: parent

        property string draggingCornerId: ""

        onEntered: {
            if (drag.keys.indexOf("resize") !== -1) {
                var newID = drag.source.cornerID;
                shotArea.startX = shotArea.x;
                shotArea.startY = shotArea.y;
                shotArea.pivot = shotArea.cornerPoint(shotArea.pivotFor(newID));
                draggingCornerId = newID;
            }
        }

        onDropped: {
            if (drop.keys.indexOf("resize") !== -1) {
                draggingCornerId = ""
                drop.source.dropped(drop);
            }
        }
    }

    ScreenshotArea {
        id: shotArea

        dragX: dropArea.drag.x
        dragY: dropArea.drag.y
        dragID: dropArea.draggingCornerId
    }

    ScreenshotAreaCorner {
        id: topLeftCorner
        cornerID: shotArea.cornerID.topLeft
        cornerPoint: shotArea.cornerPoint(cornerID)
    }

    ScreenshotAreaCorner {
        id: topRightCorner
        cornerID: shotArea.cornerID.topRight
        cornerPoint: shotArea.cornerPoint(cornerID)
    }

    ScreenshotAreaCorner {
        id: bottomRightCorner
        cornerID: shotArea.cornerID.bottomRight
        cornerPoint: shotArea.cornerPoint(cornerID)
    }

    ScreenshotAreaCorner {
        id: bottomLeftCorner
        cornerID: shotArea.cornerID.bottomLeft
        cornerPoint: shotArea.cornerPoint(cornerID)
    }
}
