import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.0
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

    Component.onCompleted: {
        App.stackView = stackView;
        root.show();
    }
}
