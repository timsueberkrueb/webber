import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Webber 1.0
import "."

Page {
    function setUrl(url) {
        urlField.text = url;
        d.refresh();
    }

    visible: false

    onVisibleChanged: {
        if (visible) {
            d.loadDefaults();
            urlField.forceActiveFocus();
        }
    }

    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            IconButton {
                iconName: "leftarrow"
                onClicked: App.pop()
            }

            TextField {
                id: urlField

                property string previousText: ""

                Layout.fillWidth: true

                placeholderText: "Url (e.g. https://example.com)"
                inputMethodHints: Qt.ImhUrlCharactersOnly
                onDisplayTextChanged: {
                    previousText = displayText;
                    if (displayText !== "") {
                        scrapeTimer.restart()
                    }
                }
                onEditingFinished: {
                    if (displayText !== previousText) {
                        scrapeTimer.stop();
                        d.refresh();
                    }
                }
            }

            Item {
                implicitHeight: urlField.height
                implicitWidth: scraper.busy ? urlField.height : 0
                clip: true

                BusyIndicator {
                    width: urlField.height
                    height: urlField.height
                    running: scraper.busy
                }

                Behavior on implicitWidth {
                    NumberAnimation {
                        duration: 500
                        easing.type: Easing.OutElastic
                    }
                }
            }
        }
    }

    ColumnLayout {
        anchors {
            fill: parent
            margins: units.gu(2)
        }

        Flickable {
            id: scrollView

            Layout.fillHeight: true
            Layout.fillWidth: true

            contentHeight: content.height
            contentWidth: scrollView.width
            interactive: content.height > height
            clip: true

            Item {
                id: content

                implicitWidth: scrollView.contentWidth
                implicitHeight: columnLayout.childrenRect.height

                ColumnLayout {
                    id: columnLayout

                    width: parent.width
                    spacing: Suru.units.dp(8)

                    Rectangle {
                        id: failLoadBox

                        visible: scraper.errorString !== "" && urlField.text !== ""

                        Layout.fillWidth: true

                        implicitHeight: childrenRect.height + Suru.units.gu(2)

                        radius: Suru.units.dp(4)
                        border.width: Suru.units.dp(1)
                        border.color: Suru.neutralColor

                        Column {
                            width: parent.width - Suru.units.gu(2)
                            x: Suru.units.gu(1)
                            y: Suru.units.gu(1)
                            spacing: Suru.units.gu(1)

                            Label {
                                width: parent.width
                                text: scraper.errorString
                                wrapMode: Text.WordWrap
                                color: Suru.color(Suru.Red)
                            }

                            RowLayout {
                                width: parent.width

                                Item {
                                    Layout.fillWidth: true
                                }

                                Button {
                                    text: "Refresh"
                                    onClicked: {
                                        d.refresh();
                                    }
                                }
                            }
                        }
                    }

                    GridLayout {
                        Layout.fillWidth: true

                        columns: 2
                        columnSpacing: Suru.units.gu(1)
                        rowSpacing: Suru.units.gu(1)

                        Label {
                            text: "Name"
                        }

                        TextField {
                            id: nameField
                            Layout.fillWidth: true
                            placeholderText: "Web app name"
                        }

                        Label {
                            text: "Color (hex)"
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

                            TextField {
                                id: colorField
                                Layout.fillWidth: true
                                text: "#ffffff"
                                validator: RegExpValidator {
                                    regExp: /^#(?:[0-9a-fA-F]{3}){1,2}$/
                                }
                            }
                        }

                        Label {
                            text: "Icon"
                        }

                        Item {
                            implicitWidth: Suru.units.gu(8)
                            implicitHeight: Suru.units.gu(8)

                            Image {
                                id: iconImage

                                anchors.fill: parent
                                sourceSize.width: Suru.units.gu(8)
                                sourceSize.height: Suru.units.gu(8)

                                BusyIndicator {
                                    anchors.centerIn: parent
                                    running: iconImage.status == Image.Loading
                                }
                            }

                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true

                        Label {
                            text: "Url patterns"
                            font.bold: true
                        }

                        Item { Layout.fillWidth: true }

                        Button {
                            text: "Add"
                            onClicked: urlPatterns.add("")
                        }
                    }

                    ListView {
                        id: urlPatternsView

                        Layout.fillWidth: true
                        implicitHeight: contentHeight
                        interactive: false

                        model: urlPatterns
                        clip: true

                        delegate: Item {
                            width: parent.width
                            height: Suru.units.gu(5)

                            RowLayout {
                                anchors.fill: parent

                                TextField {
                                    Layout.fillWidth: true
                                    text: model.url
                                    placeholderText: "http://*.example.com/*"
                                    onEditingFinished: {
                                        if (text === "") {
                                            urlPatterns.remove(index);
                                        }
                                        if (text !== "") {
                                            urlPatterns.setUrl(index, text);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Rectangle {
            Layout.bottomMargin: Suru.units.gu(1)
            Layout.fillWidth: true
            height: Suru.units.dp(1)
            color: Suru.neutralColor
        }

        RowLayout {
            Layout.fillWidth: true

            Button {
                text: "Reset"
                onClicked: d.loadDefaults()
            }

            Item { Layout.fillWidth: true }

            Button {
                text: "Create"
                enabled: urlField.text !== "" && nameField.text !== ""
                onClicked: {
                    addDialog.open();
                    appModel.create(
                        urlField.text,
                        nameField.text,
                        colorField.text,
                        scraper.iconUrl,
                        urlPatterns.getPatternsString()
                    );
                }
            }
        }
    }

    Dialog {
        id: addDialog

        x: (parent.width - width) / 2
        y: (parent.height - height) / 2

        title: "Creating shortcut ..."
        contentItem: Item {
            implicitWidth: Suru.units.dp(128)
            implicitHeight: Suru.units.dp(128)

            BusyIndicator {
                anchors.centerIn: parent
                running: true
            }
        }

        standardButtons: Dialog.NoButton
        modal: true
        closePolicy: Dialog.NoAutoClose
    }

    InstallDialog {
        id: installDialog

        x: (parent.width - width) / 2
        y: (parent.height - height) / 2

        contentWidth: parent.width - Suru.units.gu(16)
        contentHeight: parent.height - Suru.units.gu(16)

        standardButtons: Dialog.NoButton
        modal: true
        closePolicy: Dialog.NoAutoClose
    }

    Timer {
        id: scrapeTimer
        interval: 800
        repeat: false
        onTriggered: scraper.scrape()
    }

    QtObject {
        id: d

        function loadDefaults() {
            nameField.text = "";
            colorField.text = "#ffffff";
            urlField.text = "";
            iconImage.source = "";
            urlPatterns.clear();
        }

        function refresh() {
            if (urlField.displayText !== "") {
                scraper.scrape();
            }
        }
    }

    UrlPatternsModel {
        id: urlPatterns
    }

    AppModel {
        id: appModel

        onCreated: {
            addDialog.close()
            installDialog.open();
        }
    }

    WebScraper {
        id: scraper
        url: urlField.displayText
        onScraped: {
            if (siteName != "" && nameField.text !== siteName) {
                nameField.text = siteName;
            } else if (title !== "" && nameField.text !== title) {
                nameField.text = title;
            }
            if (themeColor != "" && colorField.validator.regExp.test(themeColor)) {
                colorField.text = themeColor;
            }
            iconImage.source = iconUrl !== "" ? Qt.resolvedUrl(iconUrl) : "";
            if (defaultUrlPatterns !== []) {
                urlPatterns.clear();
                for (var i=0; i<defaultUrlPatterns.length; ++i) {
                    urlPatterns.add(defaultUrlPatterns[i]);
                }
            }
        }
    }
}
