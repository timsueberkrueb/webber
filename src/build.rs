/* Copyright (C) 2018 Olivier Goffart <ogoffart@woboq.com>
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use semver::Version;

/// Generate gettext translation files
fn gettext() {
    let pot_file = "po/webber.pot";
    let source_files = source_files();

    let mut child = Command::new("xgettext")
        .args([
            &format!("--output={}", pot_file),
            "--qt",
            "--keyword=tr",
            "--keyword=tr:1,2",
            "--add-comments=i18n",
        ])
        .args(&source_files)
        .spawn()
        .unwrap();

    let exit_status = child.wait().unwrap();
    assert!(exit_status.code() == Some(0));

    for po_file in po_files() {
        let mut child = Command::new("msgmerge")
            .args(["--update", po_file.to_str().unwrap(), pot_file])
            .spawn()
            .unwrap();

        let exit_status = child.wait().unwrap();
        assert!(exit_status.code() == Some(0));

        let mo_dir = format!(
            "share/locale/{}/LC_MESSAGES",
            po_file.file_stem().unwrap().to_str().unwrap()
        );
        fs::create_dir_all(&mo_dir).unwrap();

        let mo_file = format!("{}/webber.timsueberkrueb.mo", mo_dir);

        let mut child = Command::new("msgfmt")
            .args([po_file.to_str().unwrap(), "-o", &mo_file])
            .spawn()
            .unwrap();

        let exit_status = child.wait().unwrap();
        assert!(exit_status.code() == Some(0));
    }
}

fn source_files() -> Vec<PathBuf> {
    fs::read_dir("qml")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect()
}

fn po_files() -> Vec<PathBuf> {
    fs::read_dir("po")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|ext| ext == "po").unwrap_or(false))
        .collect()
}

fn setup_qt() {
    let qt_include_path = std::env::var("DEP_QT_INCLUDE_PATH").unwrap();
    let qt_library_path = std::env::var("DEP_QT_LIBRARY_PATH").unwrap();
    let qt_version = std::env::var("DEP_QT_VERSION")
        .unwrap()
        .parse::<Version>()
        .expect("Parsing Qt version failed");

    if qt_version >= Version::new(6, 0, 0) {
        // Webber doesn't support Qt 6, yet
        println!("cargo:rustc-cfg=no_qt");
        return;
    }

    let mut config = cpp_build::Config::new();

    if cfg!(target_os = "macos") {
        config.flag("-F");
        config.flag(&qt_library_path);
    }

    config.include(&qt_include_path).build("src/main.rs");
}

fn main() {
    gettext();
    setup_qt();
}
