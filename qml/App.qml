pragma Singleton

import QtQuick 2.0

QtObject {
    property var dialogContainer: null
    property var stackView: null

    function pop() {
        if (stackView) {
            stackView.pop();
        } else {
            console.error("StackView not set");
        }
    }
}
