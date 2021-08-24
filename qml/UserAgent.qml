/*
 * Copyright 2013-2016 Canonical Ltd.
 *
 * This file was part of morph-browser.
 * Adapted from https://github.com/ubports/morph-browser,
 * git commit: 82076a1d9efc1dbdec08d44531d001b4b1b6cf6b
 *
 * morph-browser is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 3.
 *
 * webbrowser-app is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pragma Singleton

import QtQml 2.0
import QtWebEngine 1.9

/*
 * Useful documentation:
 *   http://en.wikipedia.org/wiki/User_agent#Format
 *   https://developer.mozilla.org/en-US/docs/Gecko_user_agent_string_reference
 *   https://wiki.mozilla.org/B2G/User_Agent
 *   https://github.com/mozilla-b2g/gaia/blob/master/build/ua-override-prefs.js
 *   https://developers.google.com/chrome/mobile/docs/user-agent
 */

QtObject {
    id: userAgent

    readonly property string mobile: getUA("Mobile")

    readonly property string desktop: getUA("")

    function getChromiumVersion() {
        var temporaryDefaultProfile = Qt.createQmlObject("import QtWebEngine 1.9; WebEngineProfile {offTheRecord: true}", userAgent);
        var regex = /(^| )(Chrome|Chromium)\/([0-9.]*)( |$)/;
        var chromiumVersion = temporaryDefaultProfile.httpUserAgent.match(regex)[3];
        temporaryDefaultProfile.destroy();
        return chromiumVersion;
    }

    function getUA(formFactor) {
        var ua = d.template
        ua = ua.arg((d.attributes(formFactor) !== "") ? " %1".arg(d.attributes(formFactor)) : "") // %2
        ua = ua.arg((d.hardwareID !== "") ? "; %1".arg(d.hardwareID) : "") // %3
        ua = ua.arg(d.webkitVersion) // %4
        ua = ua.arg(d.chromiumVersion) // %5
        ua = ua.arg((formFactor !== "") ? "%1 ".arg(formFactor) : "") // %6
        ua = ua.arg(d.webkitVersion) // %7
        ua = ua.arg((d.more !== "") ? " %1".arg(d.more) : "") // %8
        return ua
    }

    readonly property QtObject __private: QtObject {
        id: d

        // %1: optional token to specify further attributes of the platform, e.g. "like Android"
        // %2: optional hardware ID token
        // %3: WebKit version, e.g. "537.36"
        // %4: Chromium version, e.g. "35.0.1870.2"
        // %5: Optional token to provide additional free-form information, e.g. "Mobile"
        // %6: Safari version, e.g. "537.36"
        // %7: Optional token, in case some extra bits are needed to make things work (e.g. an extra form factor info etc.)
        //
        // note #1: "Mozilla/5.0" is misinformation, but it is a legacy token that
        //   virtually every single UA out there has, it seems unwise to remove it
        // note #2: "AppleWebKit", as opposed to plain "WebKit", does make a
        //   difference in the content served by certain sites (e.g. gmail.com)
        readonly property string template: "Mozilla/5.0 (Linux; Ubuntu %1%2) AppleWebKit/%3 Chrome/%4 %5Safari/%6%7"

        readonly property string hardwareID: ""

        // See chromium/src/content/webkit_version.h.in in oxide’s source tree.
        readonly property string webkitVersion: "537.36"

        readonly property string chromiumVersion: getChromiumVersion()

        readonly property string more: ""

        function attributes(formFactor) {
            return formFactor === "Mobile" ? "like Android 9" : "";
        }
    }
}
