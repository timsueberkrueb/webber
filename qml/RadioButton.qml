import QtQuick 2.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2

/*!
    The standard QQC2 RadioButton, but the width behaves the same whether the text is empty or not
 */
RadioButton {
    id: control

    rightPadding: 0

    Component.onCompleted: {
        indicator.x = Qt.binding(function() {
            return control.mirrored ? control.width - width - control.rightPadding : control.leftPadding;
        })
    }
}
