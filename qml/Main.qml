import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Webber 1.0
import "."

ApplicationWindow {
    id: root

    property bool landscape: width > height
    readonly property bool tablet: landscape ? width >= units.gu(90) : height >= units.gu(90)

    width: Suru.units.dp(640)
    height: Suru.units.dp(480)

    title: "Webber"
    visible: true

    StackView {
        id: stackView

        anchors {
            top: parent.top
            left: parent.left
            right: parent.right
            bottom: keyboard.top
        }

        initialItem: MainPage {}
    }

    KeyboardPlaceholder {
        id: keyboard
        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
        }
    }

    AddPage { id: addPage }

    ContentImport {
        onUrlRequested: {
            stackView.push(addPage);
            addPage.setUrl(url);
        }
    }

    HelpDialog {
        id: helpDialog

        parent: App.dialogContainer

        x: (parent.width - width) / 2
        y: (parent.height - height) / 2

        onClosed: Qt.inputMethod.hide()

        width: parent.width - Suru.units.gu(4)
        height: Math.min(implicitHeight, parent.height - Suru.units.gu(4))

        modal: true
        closePolicy: Dialog.NoAutoClose
    }

    Component.onCompleted: {
        App.dialogContainer = stackView;
        App.stackView = stackView;
        App.helpDialog = helpDialog;
        root.show();
    }
}
