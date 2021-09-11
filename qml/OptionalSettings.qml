import QtQuick 2.6
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK
import "."

Item {
    id: optionalSettings

    property var appModel
    property bool showing: false
    property alias themeColor: colorField.text
    property alias enableAddressBar: radioTitleBar.checked
    property alias enableBackForward: radioTitleBarBackForward.checked
    property alias enableFullscreen: checkFullscreen.checked
    property string userAgent: ""

    function loadDefaults() {
        colorField.text = "#ffffff";
        radioNoTitleBar.checked = true;
        checkFullscreen.checked = false;
        radioDefaultUA.checked = true;
        customUAField.text = "";
    }

    function isValidColor(color) {
        return colorField.validator.regExp.test(color);
    }

    implicitHeight: visible ? column.childrenRect.height : 0
    visible: showing

    Column {
        id: column

        width: parent.width
        height: childrenRect.height

        spacing: Suru.units.gu(1)

        Label {
            text: i18n.tr("Visuals & Behavior")
            font.bold: true
        }

        GridLayout {
            width: parent.width

            columns: 2
            columnSpacing: Suru.units.gu(1)
            rowSpacing: Suru.units.gu(1)

            Label {
                text: i18n.tr("Splash screen color (hex)")
            }

            RowLayout {
                Layout.fillWidth: true

                Rectangle {
                    implicitHeight: Suru.units.gu(4)
                    implicitWidth: Suru.units.gu(4)
                    radius: Suru.units.dp(4)
                    border.width: Suru.units.dp(1)
                    border.color: Suru.neutralColor
                    color: colorField.text
                }

                UUITK.TextField {
                    id: colorField
                    Layout.fillWidth: true
                    text: "#ffffff"
                    validator: RegExpValidator {
                        regExp: /^#(?:[0-9a-fA-F]{3}){1,2}$/
                    }
                }
            }
        }

        CheckBox {
            id: checkFullscreen
            text: i18n.tr("Fullscreen")
        }

        Label {
            text: i18n.tr("Controls")
            font.bold: true
        }

        Column {
            width: parent.width
            spacing: Suru.units.gu(1)

            RadioButton {
                id: radioNoTitleBar
                text: i18n.tr("Don't show a title bar")
                checked: true
            }

            RadioButton {
                id: radioTitleBar
                text: i18n.tr("Show title bar")
            }

            RadioButton {
                id: radioTitleBarBackForward
                text: i18n.tr("Show title bar with back/forward buttons")
            }
        }

        RowLayout {
            width: parent.width
            spacing: units.gu(1)

            Label {
                text: i18n.tr("Browser Identifier (User-Agent header)")
                font.bold: true
            }

            IconButton {
                /// i18n: %1, %2 are placeholders for urls. Translate the text inside of the <a>...</a> HTML tags. Don't change the HTML tags or placeholders!
                readonly property string helpText: i18n.tr(
                    "The User-Agent header is a string of text in a specific format that browsers send to servers to identify themselves. \
It contains technical information about the browser, operating system and vendor. \
Unfortunately, some websites restrict access or features based on the User-Agent header. \
To workaround this <a href=\"%1\">bad practice</a>, \
you can <a href=\"%2\">choose a common User-Agent header</a> that your website accepts \
and copy it into the text field below.")
                        .arg("https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent")
                        .arg("https://user-agents.net")
                iconName: "help"
                onClicked: App.showHelp(i18n.tr("User-Agent header"), helpText, "https://user-agents.net")
            }

            Item { Layout.fillWidth: true }
        }

        Grid {
            width: parent.width

            columns: 2
            columnSpacing: 0
            rowSpacing: Suru.units.gu(1)

            RadioButton {
                id: radioDefaultUA
                checked: true
                onCheckedChanged: {
                    if (checked) {
                        userAgent = "";
                    }
                }
            }

            Label {
                text: i18n.tr("Auto (default)")

                MouseArea {
                    anchors.fill: parent
                    onClicked: radioDefaultUA.checked = true
                }
            }

            RadioButton {
                id: radioMobileUA

                onCheckedChanged: {
                    if (checked) {
                        userAgent = UserAgent.mobile;
                    }
                }
            }

            Label {
                text: i18n.tr("Mobile")

                MouseArea {
                    anchors.fill: parent
                    onClicked: radioMobileUA.checked = true
                }
            }

            RadioButton {
                id: radioDesktopUA

                onCheckedChanged: {
                    if (checked) {
                        userAgent = UserAgent.desktop;
                    }
                }
            }

            Label {
                text: i18n.tr("Desktop")

                MouseArea {
                    anchors.fill: parent
                    onClicked: radioDesktopUA.checked = true
                }
            }

            RadioButton {
                id: radioCustomUA
            }

            Item {
                width: parent.width
                implicitHeight: customUAField.implicitHeight

                UUITK.TextField {
                    id: customUAField
                    placeholderText: i18n.tr("Custom User-Agent header")
                    enabled: radioCustomUA.checked
                    height: parent.height
                    width: parent.width

                    Binding {
                        target: optionalSettings
                        when: customUAField.enabled
                        property: "userAgent"
                        value: customUAField.text
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    enabled: !radioCustomUA.checked
                    onClicked: {
                        radioCustomUA.checked = true;
                        radioCustomUA.forceActiveFocus();
                    }
                }
            }
        }

        RowLayout {
            width: parent.width

            spacing: units.gu(1)

            Label {
                text: i18n.tr("Url patterns")
                font.bold: true
            }

            IconButton {
                iconName: "help"
                onClicked: Qt.openUrlExternally("http://docs.ubports.com/en/latest/appdev/webapp/guide.html#url-patterns")
            }

            Item { Layout.fillWidth: true }

            Button {
                text: i18n.tr("Add")
                onClicked: appModel.urlPatterns.add("")
            }
        }

        ListView {
            id: urlPatternsView

            width: parent.width
            implicitHeight: contentHeight
            interactive: false

            model: appModel.urlPatterns.model
            clip: true

            delegate: Item {
                width: parent.width
                height: Suru.units.gu(5)

                RowLayout {
                    anchors.fill: parent

                    UUITK.TextField {
                        Layout.fillWidth: true

                        signal editingFinished()

                        text: model.url
                        placeholderText: "http://*.example.com/*"
                        inputMethodHints: Qt.ImhUrlCharactersOnly

                        onAccepted: editingFinished()
                        onActiveFocusChanged: if (!activeFocus) editingFinished()
                        onEditingFinished: {
                            if (text === "") {
                                appModel.urlPatterns.remove(index);
                            }
                            if (text !== "") {
                                appModel.urlPatterns.setUrl(index, text);
                            }
                        }
                    }
                }
            }
        }

        RowLayout {
            width: parent.width
            spacing: units.gu(1)

            Label {
                text: i18n.tr("Permissions")
                font.bold: true
            }

            IconButton {
                iconName: "help"
                onClicked: Qt.openUrlExternally("http://docs.ubports.com/en/latest/appdev/platform/apparmor.html")
            }

            Item { Layout.fillWidth: true }
        }

        ListView {
            width: parent.width
            implicitHeight: contentHeight
            interactive: false

            model: appModel.permissions.model
            clip: true

            delegate: ItemDelegate {
                width: parent.width
                height: Suru.units.gu(5)

                onClicked: {
                    checkbox.toggle();
                    appModel.permissions.setEnabled(index, checkbox.checked);
                }

                RowLayout {
                    anchors.fill: parent

                    CheckBox {
                        id: checkbox
                        checked: model.enabled
                        onToggled: {
                            appModel.permissions.setEnabled(index, checked);
                        }

                        Connections {
                            target: model
                            onEnabledChanged: {
                                checkbox.checked = model.enabled;
                            }
                        }
                    }

                    Label {
                        text: model.description
                    }

                    Item { Layout.fillWidth: true }
                }
            }
        }
    }
}
