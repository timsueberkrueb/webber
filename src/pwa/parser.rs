use std::fmt;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

use csscolorparser::Color;
use language_tags::LanguageTag;

use crate::resolvable::*;
use crate::serde_utils::*;

/// Web app manifests are part of a collection of web technologies called progressive web apps
/// (PWAs), which are websites that can be installed to a device’s homescreen without an app store.
/// Unlike regular web apps with simple homescreen links or bookmarks, PWAs can be downloaded in
/// advance and can work offline, as well as use regular Web APIs.
/// See [the Web app manifests MDN docs](https://developer.mozilla.org/en-US/docs/Web/Manifest) for
/// more information.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Manifest<R: ResolveType> {
    /// The manifest's background_color member describes the expected background color of the web
    /// application. It repeats what is already available in the application stylesheet but can be
    /// used by the user agent to draw the background color of a web application for which the
    /// manifest is known before the files are actually available, whether they are fetched from
    /// the network or retrieved from disk.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub background_color: Option<Color>,
    /// The manifest's dir member specifies the base direction for the localizable members of the
    /// manifest. The dir member's value can be set to a text-direction.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub dir: Option<TextDirection>,
    /// The manifest's display member represents the developer's preferred display mode for the web
    /// application. Its value is a display mode.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub display: Option<DisplayMode>,
    /// The manifest's icons member are images that serve as iconic representations of the web
    /// application in various contexts. For example, they can be used to represent the web
    /// application amongst a list of other applications, or to integrate the web application with
    /// an OS's task switcher and/or system preferences.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub icons: Option<R::Array<Icon<R>>>,
    /// The manifest's lang member is a string in the form of a language tag that specifies the
    /// primary language for the values of the manifest's localizable members (as knowing the
    /// language can also help with directionality).
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub lang: Option<LanguageTag>,
    /// The manifest's name member is a string that represents the name of the web application as it
    /// is usually displayed to the user (e.g., amongst a list of other applications, or as a label
    /// for an icon).
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub name: Option<String>,
    /// The manifest's orientation member is a string that serves as the default screen orientation
    /// for all top-level browsing contexts of the web application.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub orientation: Option<Orientation>,
    /// The manifest's scope member is a string that represents the navigation scope of this web
    /// application's application context.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub scope: Option<R::Url>,
    /// The manifest's short_name member is a string that represents a short version of the name of
    /// the web application. It is intended to be used where there is insufficient space to display
    /// the full name of the web application.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub short_name: Option<String>,
    /// The manifest's shortcuts member is an list of shortcut items that provide access to key
    /// tasks within a web application.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub shortcuts: Option<R::Array<ShortcutItem<R>>>,
    /// The manifest's start_url member is a string that represents the start URL, which is URL that
    /// the developer would prefer the user agent load when the user launches the web application
    /// (e.g., when the user clicks on the icon of the web application from a device's application
    /// menu or homescreen).
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub start_url: Option<R::Url>,
    /// The manifest's theme_color member serves as the default theme color for an application
    /// context.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub theme_color: Option<Color>,
}

impl Manifest<Unresolved> {
    pub fn parse(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct ShortcutItem<R: ResolveType> {
    /// The shortcut item's name member is a string that represents the name of the shortcut as it
    /// is usually displayed to the user in a context menu.
    pub name: String,
    /// The shortcut item's short_name member is a string that represents a short version of the
    /// name of the shortcut. It is intended to be used where there is insufficient space to display
    /// the full name of the shortcut.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub short_name: Option<String>,
    /// The shortcut item's description member is a string that allows the developer to describe
    /// the purpose of the shortcut. User agents MAY expose this information to assistive
    /// technology.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub description: Option<String>,
    /// The shortcut item's url member is a URL within scope of a processed manifest that opens when
    /// the associated shortcut is activated.
    pub url: R::Url,
    /// The shortcut item's icons member lists images that serve as iconic representations of the
    /// shortcut in various contexts.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    pub icons: Option<R::Array<Icon<R::This>>>,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub enum Orientation {
    /// Any is an orientation that means the screen can be locked to any one of portrait-primary,
    /// portrait-secondary, landscape-primary and landscape-secondary.
    #[serde(rename = "any")]
    Any,
    /// Natural is an orientation that refers to either portrait-primary or landscape-primary
    /// depending on the device's usual orientation. This orientation is usually provided by the
    /// underlying operating system.
    #[serde(rename = "natural")]
    Natural,
    /// Landscape is an orientation where the screen width is greater than the screen height and
    /// depending on platform convention locking the screen to landscape can represent landscape-primary,
    /// landscape-secondary or both.
    #[serde(rename = "landscape")]
    Landscape,
    /// Portrait is an orientation where the screen width is less than or equal to the screen height
    /// and depending on platform convention locking the screen to portrait can represent
    /// portrait-primary, portrait-secondary or both.
    #[serde(rename = "portrait")]
    Portrait,
    /// Portrait-primary is an orientation where the screen width is less than or equal to the
    /// screen height. If the device's natural orientation is portrait, then it is in
    /// portrait-primary when held in that position. If the device's natural orientation is
    /// landscape, the user agent sets portrait-primary at either angle 90 or angle 270.
    #[serde(rename = "portrait-primary")]
    PortraitPrimary,
    /// Portrait-secondary is an orientation where the screen width is less than or equal to the
    /// screen height. If the device's natural orientation is portrait, then it is in
    /// portrait-secondary when rotated 180º from its natural position. If the device's natural
    /// orientation is landscape, the user agent sets portrait-secondary at an angle not used for
    /// portrait-primary.
    #[serde(rename = "portrait-secondary")]
    PortraitSecondary,
    /// Landscape-primary is an orientation where the screen width is greater than the screen
    /// height. If the device's natural orientation is landscape, then it is in landscape-primary
    /// when held in that position. If the device's natural orientation is portrait, the user agent
    /// sets landscape-primary at either angle 90 or angle 270.
    #[serde(rename = "landscape-primary")]
    LandscapePrimary,
    /// Landscape-secondary is an orientation where the screen width is greater than the screen
    /// height. If the device's natural orientation is landscape, it is in landscape-secondary when
    /// rotated 180º from its natural orientation. If the device's natural orientation is portrait,
    /// the user agent sets landscape-secondary at the angle not used for landscape-primary.
    #[serde(rename = "landscape-secondary")]
    LandscapeSecondary,
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct Icon<R: ResolveType> {
    /// A list of image dimensions
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_default")]
    pub sizes: IconSizes,
    /// The path to the image file. If src is a relative URL, the base URL will be the URL of the
    /// manifest.
    pub src: R::Url,
    /// A hint as to the media type of the image. The purpose of this member is to allow a user
    /// agent to quickly ignore images with media types it does not support.
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_none")]
    #[serde(rename = "type")]
    pub icon_type: Option<String>,
    /// When a manifest image resource is used as an icon, a developer can hint that the image is
    /// intended to serve some special purpose in the context of the host OS (i.e., for better
    /// integration).
    // FIXME: We are a bit less strict than the spec here:
    // If the list of purposes is empty, the spec defines that failure should be returned.
    // Instead, we will return the default value due to our use of `#[serde(default)]`.
    // https://w3c.github.io/manifest/#dfn-determine-the-purpose-of-an-image
    #[serde(default)]
    #[serde(deserialize_with = "ok_or_default")]
    pub purpose: IconPurpose,
}

#[derive(Debug, Default)]
pub struct IconSizes {
    pub sizes: Vec<(u64, u64)>,
}

impl<'de> Deserialize<'de> for IconSizes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = IconSizes;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "an sequence of purpose keywords separated by ASCII whitespace"
                )
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                let mut sizes = Vec::new();
                for size in v.split_ascii_whitespace() {
                    match size.split('x').collect::<Vec<_>>().as_slice() {
                        [w, h] => match (w.parse(), h.parse()) {
                            (Ok(w), Ok(h)) => sizes.push((w, h)),
                            _ => continue,
                        },
                        _ => continue,
                    }
                }
                Ok(IconSizes { sizes })
            }
        }

        deserializer.deserialize_str(V)
    }
}

/// Defines the purpose of the image, for example if the image is intended to serve some special
/// purpose in the context of the host OS (i.e., for better integration).
#[derive(Debug)]
#[non_exhaustive]
pub struct IconPurpose {
    /// A user agent can present this icon where a monochrome icon with a solid fill is needed. The
    /// color information in the icon is discarded and only the alpha data is used. The icon can
    /// then be used by the user agent like a mask over any solid fill.
    pub monochrome: bool,
    /// The image is designed with icon masks and safe zone in mind, such that any part of the image
    /// that is outside the safe zone can safely be ignored and masked away by the user agent.
    pub maskable: bool,
    /// The user agent is free to display the icon in any context.
    pub any: bool,
}

impl Default for IconPurpose {
    fn default() -> Self {
        Self {
            any: true,
            ..Self::empty()
        }
    }
}

impl IconPurpose {
    fn empty() -> Self {
        Self {
            monochrome: false,
            maskable: false,
            any: false,
        }
    }

    fn is_empty(&self) -> bool {
        !(self.monochrome || self.maskable || self.any)
    }
}

impl<'de> Deserialize<'de> for IconPurpose {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = IconPurpose;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "an sequence of purpose keywords separated by ASCII whitespace"
                )
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                let keywords = v.split_ascii_whitespace();
                let mut purpose = IconPurpose::empty();
                for keyword in keywords {
                    match keyword {
                        "monochrome" => purpose.monochrome = true,
                        "maskable" => purpose.maskable = true,
                        "any" => purpose.any = true,
                        _ => continue,
                    }
                }
                if purpose.is_empty() {
                    return Err(serde::de::Error::custom(
                        "Icon purpose field contains no valid keywords",
                    ));
                }
                Ok(purpose)
            }
        }

        deserializer.deserialize_str(V)
    }
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub enum TextDirection {
    /// Left-to-right text.
    #[serde(rename = "ltr")]
    LeftToRight,
    /// Right-to-left text.
    #[serde(rename = "rtl")]
    RightToLeft,
    /// No explicit directionality.
    #[serde(rename = "auto")]
    Auto,
}

///  A display mode represents how the web application is being presented within the context of an
/// OS (e.g., in fullscreen, etc.). Display modes correspond to user interface (UI) metaphors and
/// functionality in use on a given platform. The UI conventions of the display modes are purely
/// advisory and implementers are free to interpret them how they best see fit.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub enum DisplayMode {
    /// Opens the web application with browser UI elements hidden and takes up the entirety of the
    /// available display area.
    #[serde(rename = "fullscreen")]
    Fullscreen,
    /// Opens the web application to look and feel like a standalone native application. This can
    /// include the application having a different window, its own icon in the application launcher,
    /// etc. In this mode, the user agent will exclude standard browser UI elements such as an URL
    /// bar, but can include other system UI elements such as a status bar and/or system back
    /// button.
    #[serde(rename = "standalone")]
    Standalone,
    /// This mode is similar to standalone, but provides the end-user with some means to access a
    /// minimal set of UI elements for controlling navigation (i.e., back, forward, reload, and
    /// perhaps some way of viewing the document's address). A user agent can include other platform
    /// specific UI elements, such as "share" and "print" buttons or whatever is customary on the
    /// platform and user agent.
    #[serde(rename = "minimal-ui")]
    MinimalUI,
    /// Opens the web application using the platform-specific convention for opening hyperlinks in
    /// the user agent (e.g., in a browser tab or a new window).
    #[serde(rename = "browser")]
    Browser,
}
