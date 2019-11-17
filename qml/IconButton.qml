import QtQuick 2.0
import QtQuick.Controls 2.0
import QtQuick.Controls.Suru 2.2

Button {
    property string iconName: ""

    width: implicitWidth
    height: implicitWidth
    implicitWidth: Suru.units.gu(4)
    implicitHeight: Suru.units.gu(4)
    flat: true

    Image {
        anchors {
            fill: parent
            margins: Suru.units.gu(1)
        }

        sourceSize.width: width
        sourceSize.height: height
        source: "image://suru/" + iconName
    }
}
