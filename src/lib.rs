//! Bootstrap icons for yew
//!
//! # Version
//!
//! This release is for yew 0.2.0 and contains bootstrap-icons-v1.10.5.
//!
//! # Icons
//!
//! All icons are available as a constant:
//! ```
//! # use yew::html;
//! # use yew_bootstrap_icons::BI;
//! let icon = BI::HEART;
//! let empty_icon = BI::empty(); // or BI::default()
//! # let result =
//! html!{
//!     <h1>{"I"} {icon} {BI::GEAR}</h1>
//! }
//! # ;
//! ```
//!
//! # Files
//!
//! The files can be added though several ways:
//!
//! * Copy them yourself from the website
//! * Use [`BIFiles::cdn()`](crate::BIFiles::cdn)
//! * Use [`BIFiles::copy()`](crate::BIFiles::copy) - see below
//! * Access the data via [`BIFiles::FILES`](crate::BIFiles::FILES) and deliver them yourself
//!
//! # Automatically copy the files
//!
//! There are some options, two are explained below.
//!
//! 0. Either way you now have to specify your wasm-program in `index.html`:
//!    ```html
//!    <link data-trunk rel="rust" data-bin="name-of-app" />
//!    ```
//!    (Because there are now two binaries and trunk can't decide.)
//!
//! 1. Add a binary to your `Cargo.toml`
//!    ```toml
//!    [[bin]]
//!    name = "copy-bootstrap-icons"
//!    ```
//!
//! # Option 1: Copy to dist
//!
//! 2. Create the file `src/bin/copy-bootstrap-icons.rs` with:
//!    ```no_run
//!    use std::path::PathBuf;
//!    use yew_bootstrap_icons::BIFiles;
//!
//!    fn main() -> Result<(), std::io::Error> {
//!        let path = PathBuf::from(
//!            std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
//!        )
//!        .join(BIFiles::NAME);
//!        if !path.is_dir() {
//!            std::fs::create_dir(&path)?;
//!        }
//!        BIFiles::copy(&path)
//!    }
//!    ```
//!
//! 3. Add the css to your `index.html`
//!    ```html
//!    <link rel="stylesheet" href="bootstrap-icons-v1.10.5/bootstrap-icons.css" />
//!    ```
//!    (Don't forget to set `<base data-trunk-public-url />`.)
//!
//! 4. Add the program to your `Trunk.toml`
//!    ```toml
//!    [[hooks]]
//!    stage = "build"
//!    command = "cargo"
//!    command_arguments = ["run", "--bin", "copy-bootstrap-icons"]
//!    ```
//!
//! # Option 2: Copy to source (and let trunk copy it to dist)
//!
//! This means that trunk will add the hash to the css-file.
//!
//! It is assumed that your directory for static files is called `static`, if not
//! change the paths below.
//!
//! 2. Create the file `src/bin/copy-bootstrap-icons.rs` with:
//!    ```no_run
//!    use std::path::PathBuf;
//!    use yew_bootstrap_icons::BIFiles;
//!
//!    fn main() -> Result<(), std::io::Error> {
//!        let path = &PathBuf::from(
//!            std::env::var("TRUNK_SOURCE_DIR").expect("Environment variable TRUNK_SOURCE_DIR"),
//!        )
//!        .join("static")
//!        .join(BIFiles::NAME);
//!        if !path.is_dir() {
//!            std::fs::create_dir(&path)?;
//!        }
//!        BIFiles::copy(&path)
//!    }
//!    ```
//!
//! 3. Add the css to your `index.html`
//!    ```html
//!    <link data-trunk rel="css" href="static/bootstrap-icons-v1.10.5/bootstrap-icons.css" />
//!    <link data-trunk rel="copy-dir" href="static/bootstrap-icons-v1.10.5/fonts" />
//!    ```
//!
//! 4. Add the program to your `Trunk.toml`
//!    ```toml
//!    [[hooks]]
//!    stage = "pre_build"
//!    command = "cargo"
//!    command_arguments = ["run", "--bin", "copy-bootstrap-icons"]
//!    ```
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

use core::hash::{Hash, Hasher};
use yew::html::{ChildrenRenderer, IntoPropValue};
use yew::virtual_dom::{VNode, VRaw};
use yew::{AttrValue, Html};

/// Represents an bootstrap-icon.
///
/// (Or nothing, see [`BI::empty()`]/[`BI::default()`].)
///
/// It's a transparent wrapper of a `&'static str`, so `Copy` is cheap.
///
/// Use [`BI::html`] or the `From` or `IntoPropValue` implementation to generate the actual html.
///
/// Search for an icon at <https://icons.getbootstrap.com/#search>.
// Invariant: All possible strings are different and thus `(ptr,len)` must me different as well.
// Invariant: No two strings at different pointers are equal,
// Invariant: this is guaranteed due to the fact that it's not possible to create new.
#[derive(Clone, Copy, Ord, PartialOrd, Eq)]
#[repr(transparent)]
pub struct BI(pub(crate) &'static str);

impl BI {
    /// Create an empty `BI`.
    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        BI("")
    }

    /// Returns true is self is empty.
    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0.is_empty()
    }

    /// Returns the `Html` of this icon.
    #[inline]
    #[must_use]
    pub const fn html(self) -> Html {
        VNode::VRaw(VRaw {
            html: AttrValue::Static(self.0),
        })
    }

    /// Returns the raw html as a str of this icon.
    #[inline]
    #[must_use]
    pub const fn raw_html(self) -> &'static str {
        self.0
    }
}

impl PartialEq for BI {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Invariant: All possible strings are different and thus `(ptr,len)` must me different as well.
        // Invariant: No two strings at different pointers are equal,
        // Invariant: this is guaranteed due to the fact that it's not possible to create new.
        // Performance hack: Only check those.
        self.0.as_ptr() as usize == other.0.as_ptr() as usize && self.0.len() == other.0.len()
    }
}

impl Hash for BI {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Invariant: All possible strings are different and thus `(ptr,len)` must me different as well.
        // Invariant: No two strings at different pointers are equal,
        // Invariant: this is guaranteed due to the fact that it's not possible to create new.
        // Performance hack: Only hash the ptr to the middle of the string.
        (self.0.as_ptr() as usize + (self.0.len() >> 1)).hash(state);
    }
}

impl Default for BI {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl From<BI> for Html {
    #[inline]
    fn from(value: BI) -> Self {
        value.html()
    }
}

impl From<&BI> for Html {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn from(value: &BI) -> Self {
        Html::from(*value)
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for BI {
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        self.html().into_prop_value()
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for &BI {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        (*self).into_prop_value()
    }
}

/// Holds all bootstrap-icons data.
///
/// Intended use:
/// ```
/// # use yew_bootstrap_icons::BIFiles;
/// let BIFiles {css, font_woff, font_woff2} = BIFiles::FILES;
/// ```
/// (That way it will be an error if a file is added/removed.)
pub struct BIFiles {
    /// Contents of the file `bootstrap-icons.css`.
    pub css: &'static str,
    /// Contents of the file `fonts/bootstrap-icons.woff`.
    pub font_woff: &'static [u8],
    /// Contents of the file `fonts/bootstrap-icons.woff2`.
    pub font_woff2: &'static [u8],
}

impl BIFiles {
    /// Version of the package.
    pub const VERSION: &'static str = "v1.10.5";

    /// Name of the package.
    pub const NAME: &'static str = "bootstrap-icons-v1.10.5";

    /// All bootstrap-icons files.
    ///
    /// Intended use:
    /// ```
    /// # use yew_bootstrap_icons::BIFiles;
    /// let BIFiles {css, font_woff, font_woff2} = BIFiles::FILES;
    /// ```
    /// (That way it will be an error if a file is added/removed.)
    pub const FILES: Self = Self {
        css: include_str!("../bootstrap-icons-v1.10.5/bootstrap-icons.css"),
        font_woff: include_bytes!("../bootstrap-icons-v1.10.5/fonts/bootstrap-icons.woff"),
        font_woff2: include_bytes!("../bootstrap-icons-v1.10.5/fonts/bootstrap-icons.woff2"),
    };

    /// Load the bootstrap-icons files from the official cdn.
    ///
    /// Place `{BIFiles::cdn()}` to the top of the top level html! in your application.
    #[must_use]
    pub const fn cdn() -> VNode {
        VNode::VRaw(VRaw {
            html: AttrValue::Static(
                r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.css">"#,
            ),
        })
    }

    /// Copy all bootstrap icons files to the specified directory.
    ///
    /// # Errors
    ///
    /// Will return an error when there is a problem with creating the directories or writing the files.
    #[cfg(feature = "std")]
    pub fn copy(to: &std::path::Path) -> Result<(), std::io::Error> {
        let BIFiles {
            css,
            font_woff,
            font_woff2,
        } = Self::FILES;

        let fonts = to.join("fonts");
        if !fonts.is_dir() {
            std::fs::create_dir(&fonts)?;
        }
        std::fs::write(to.join("bootstrap-icons.css"), css)?;
        std::fs::write(fonts.join("bootstrap-icons.woff"), font_woff)?;
        std::fs::write(fonts.join("bootstrap-icons.woff2"), font_woff2)?;

        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
