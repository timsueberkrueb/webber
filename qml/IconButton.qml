import QtQuick 2.0
import QtQuick.Controls 2.0
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK

Button {
    property string iconName: ""

    width: implicitWidth
    height: implicitWidth
    implicitWidth: Suru.units.gu(4)
    implicitHeight: Suru.units.gu(4)
    flat: true

    UUITK.Icon {
        anchors {
            fill: parent
            margins: Suru.units.gu(1)
        }

        name: iconName
    }
}
