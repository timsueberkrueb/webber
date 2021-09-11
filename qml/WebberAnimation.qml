import QtQuick 2.0
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK

Item {
    id: item

    property bool running: false

    function start() {
        d.play("drop");
    }

    function startDelayed(delay) {
        animTimer.interval = delay;
        animTimer.start();
    }

    function stop() {
        d.stop();
    }

    onRunningChanged: {
        if (running) {
            startDelayed(500);
        } else {
            stop();
        }
    }

    Timer {
        id: animTimer
        running: false
        interval: 500
        onTriggered: item.start()
    }

    Repeater {
        model: d.animations
        delegate: Component {
            AnimatedImage {
                anchors {
                    horizontalCenter: parent.horizontalCenter
                }

                verticalAlignment: Image.AlignTop

                y: - (920/sourceSize.height) * paintedHeight

                readonly property string anim: d.getAnimationNameByIndex(index)

                visible: d.currentAnim == anim
                currentFrame: 0
                playing: d.currentAnim == anim
                fillMode: Image.PreserveAspectFit
                source: d.getSource(anim)
                width: parent.width
                height: sourceSize.height
                asynchronous: true
                smooth: false

                onCurrentFrameChanged: {
                    if (currentFrame === frameCount - 1) {
                        d.currentAnimationFinished();
                    }
                }
            }
        }
    }

    QtObject {
        id: d

        property var animations: ["drop", "swing", "hi"]
        property var sources:  {
            "drop": "qrc:/assets/drop_anim.gif",
            "swing": "qrc:/assets/swing_anim.gif",
            "hi": "qrc:/assets/hi_anim.gif",
        }

        property string currentAnim: ""

        function play(anim) {
            if (anim == "") {
                stop();
                return;
            }
            currentAnim = anim;
        }

        function stop() {
            currentAnim = "";
        }

        function getAnimationNameByIndex(index) {
            return animations[index];
        }

        function getSource(anim) {
            return sources[anim];
        }

        signal currentAnimationFinished()

        onCurrentAnimationFinished: {
            if (currentAnim === "drop" || currentAnim == "hi") {
                play("swing");
            } else if (currentAnim == "swing") {
                play("hi");
            }
        }
    }
}
