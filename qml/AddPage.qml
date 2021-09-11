import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 2.2
import QtQuick.Controls.Suru 2.2
import Ubuntu.Components 1.3 as UUITK
import Webber 1.0
import "."

Page {
    function setUrl(url) {
        d.loadDefaults();
        urlField.text = url;
        urlField.forceActiveFocus();
        d.refresh();
    }

    visible: false

    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            IconButton {
                iconName: "go-previous"
                onClicked: App.pop()
            }

            UUITK.TextField {
                id: urlField

                property string previousText: ""
                signal editingFinished()

                Layout.fillWidth: true

                /// i18n: %1 is a placeholder for an example url. Do not change the %1!
                placeholderText: i18n.tr("Url (e.g. %1)").arg("https://example.com")
                inputMethodHints: Qt.ImhUrlCharactersOnly
                onAccepted: editingFinished()
                onActiveFocusChanged: if (!activeFocus) editingFinished()
                onEditingFinished: {
                    if (displayText !== previousText) {
                        // Prepend https by default
                        if (displayText.trim() !== "" && displayText.trim().indexOf("http") !== 0) {
                            text = "https://" + displayText.trim();
                        }
                        previousText = displayText;
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
                implicitHeight: column.height

                Column {
                    id: column

                    width: parent.width
                    height: childrenRect.height
                    spacing: Suru.units.dp(8)

                    EssentialSettings {
                        id: essentialSettings
                        width: parent.width
                        url: urlField.text
                        iconUrl: selectIconDialog.selectedIconUrl
                        scraper: scraper
                        appModel: appModel
                        customIconSource: customIconSelector.source
                        onSelectIconRequested: selectIconDialog.open()
                        onCustomIconRequested: customIconSelector.open()
                        onScreenshotRequested: screenshotDialog.open()
                    }

                    ItemDelegate {
                        width: parent.width
                        implicitHeight: units.gu(5)

                        onClicked: optionalSettings.showing = !optionalSettings.showing

                        RowLayout {
                            anchors {
                                fill: parent
                                margins: Suru.units.gu(1)
                            }

                            spacing: units.gu(1)

                            UUITK.Icon {
                                name: optionalSettings.showing ? "up" : "down"
                                width: units.gu(3)
                                height: units.gu(3)
                            }

                            Label {
                                text: i18n.tr("Customize")
                                font.bold: true
                            }

                            Item { Layout.fillWidth: true }
                        }
                    }

                    OptionalSettings {
                        id: optionalSettings

                        width: parent.width


                        appModel: appModel
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
                text: i18n.tr("Reset")
                onClicked: d.loadDefaults()
            }

            Item { Layout.fillWidth: true }

            Button {
                text: i18n.tr("Create")
                enabled: urlField.text !== "" && essentialSettings.name !== ""
                onClicked: {
                    addDialog.open();
                    appModel.create();
                }
            }
        }
    }

    CustomIconSelector {
        id: customIconSelector

        parent: App.dialogContainer

        x: (parent.width - width) / 2
        y: (parent.height - height) / 2

        width: parent.width - units.gu(4)
        height: parent.height - units.gu(4)
    }

    SelectIconDialog {
        id: selectIconDialog
    }

    Dialog {
        id: addDialog

        parent: App.dialogContainer

        x: (parent.width - width) / 2
        y: (parent.height - height) / 2

        title: i18n.tr("Creating shortcut ...")
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

        parent: App.dialogContainer

        x: (parent.width - width) / 2
        y: (parent.height - height) / 2

        standardButtons: Dialog.NoButton
        modal: true
        closePolicy: Dialog.NoAutoClose
        clickPath: appModel.clickPath
    }

    ScreenshotDialog {
        id: screenshotDialog

        parent: App.dialogContainer

        x: (parent.width - width) / 2
        y: (parent.height - height) / 2

        screenshotPath: appModel.screenshotIconPath
        onScreenshotMade: essentialSettings.screenshotMade()
        onClosed: Qt.inputMethod.hide()

        width: parent.width - Suru.units.gu(4)
        height: parent.height - Suru.units.gu(4)

        modal: true
        closePolicy: Dialog.NoAutoClose
    }

    QtObject {
        id: d

        function loadDefaults() {
            urlField.text = "";
            urlField.previousText = "";

            essentialSettings.loadDefaults();
            optionalSettings.loadDefaults();

            appModel.urlPatterns.clear();
            loadPermissions();

            customIconSelector.source = Qt.resolvedUrl("");
            selectIconDialog.resetSelected();
        }

        function loadPermissions() {
            var perms = [
                ["audio", i18n.tr("Play audio"), true],
                ["content_exchange", i18n.tr("Upload files from other apps"), true],
                ["content_exchange_source",i18n.tr("Export files to other apps"), false],
                ["keep-display-on", i18n.tr("Keep the screen on"), false],
                ["location", i18n.tr("Access your location"), false],
                ["camera", i18n.tr("Access your camera"), false],
                ["microphone", i18n.tr("Acess your microphone"), false],
                ["sensores", i18n.tr("Access your sensors"), false]
            ];

            appModel.permissions.clear();

            for (var i=0; i<perms.length; ++i) {
                var perm = perms[i];
                appModel.permissions.add(perm[0], perm[1], perm[2]);
            }
        }

        function refresh() {
            if (urlField.displayText !== "") {
                screenshotDialog.url = urlField.text;
                scraper.scrape();
                essentialSettings.refresh();
            }
        }
    }

    AppModel {
        id: appModel

        url: urlField.text
        name: essentialSettings.name
        iconUrl: selectIconDialog.selectedIconUrl
        themeColor: optionalSettings.themeColor
        enableAddressBar: optionalSettings.enableAddressBar
        enableBackForward: optionalSettings.enableBackForward
        enableFullscreen: optionalSettings.enableFullscreen
        userAgent: optionalSettings.userAgent
        useScreenshotIcon: essentialSettings.useScreenshotIcon
        useCustomIcon: essentialSettings.useCustomIcon
        customIconPath: {
            var url = customIconSelector.source.toString();
            if (url.indexOf("file://") == 0) {
                url = url.slice("file://".length);
            }
            return url;
        }

        Component.onCompleted: {
            appModel.permissions.loadDefaults()
        }

        onCreated: {
            addDialog.close()
            installDialog.open();
        }
    }

    WebScraper {
        id: scraper
        url: urlField.displayText
        onScraped: {
            if (siteName != "") {
                essentialSettings.name = siteName;
            } else if (title !== "") {
                essentialSettings.name = title;
            }
            if (themeColor != "" && optionalSettings.isValidColor(themeColor)) {
                optionalSettings.themeColor= themeColor;
            }
            if (defaultUrlPatterns !== []) {
                appModel.urlPatterns.clear();
                for (var i=0; i<defaultUrlPatterns.length; ++i) {
                    appModel.urlPatterns.add(defaultUrlPatterns[i]);
                }
            }

            if (icons !== []) {
                IconModel.clear();
                for (var i=0; i<icons.length; ++i) {
                    IconModel.add(icons[i]);
                }
            }
        }
    }
}
