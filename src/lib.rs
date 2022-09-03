//! Utility library for [`std::path::Path`] and [`std::path::PathBuf`].
//!
//! ```
//! use std::path::Path;
//! use path_utils::PathExt;
//!
//! let path = Path::new("file.tar.gz");
//! let extensions = path.extensions_lossy().collect::<Vec<String>>();
//! assert_eq!(extensions, ["gz", "tar"]);
//! ```

#![deny(clippy::all, missing_docs)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::path::{Path, PathBuf};

/// [`Path`] and [`PathBuf`] extensions.
pub trait PathExt {
    /// Returns an iterator over the extensions of a path, lossily converted.
    ///
    /// ```
    /// use std::path::Path;
    /// use path_utils::PathExt;
    ///
    /// let path = Path::new("file.tar.gz");
    /// let extensions = path.extensions_lossy().collect::<Vec<String>>();
    /// assert_eq!(extensions, ["gz", "tar"]);
    fn extensions_lossy(&self) -> ExtensionsLossy;
}

impl<T> PathExt for T
where
    T: AsRef<Path>,
{
    fn extensions_lossy(&self) -> ExtensionsLossy {
        ExtensionsLossy {
            path: self.as_ref().to_path_buf(),
        }
    }
}

/// An iterator over the [`Path::extension`]s of a path, lossily converted.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ExtensionsLossy {
    path: PathBuf,
}

impl Iterator for ExtensionsLossy {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let s = self
            .path
            .extension()
            .map(|s| String::from(s.to_string_lossy()));
        self.path.set_extension("");
        s
    }
}

// ----------------------------------------------------------------------------
// tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn exts_lossy(p: impl AsRef<Path>) -> Vec<String> {
        p.as_ref().extensions_lossy().collect()
    }

    #[test]
    fn extensions_lossy() {
        assert!(Path::new("/path/to/file")
            .extensions_lossy()
            .next()
            .is_none());

        assert!(PathBuf::from("/path/to/.file")
            .extensions_lossy()
            .next()
            .is_none());

        assert_eq!(exts_lossy("/path/to/file.tar"), ["tar"]);
        assert_eq!(exts_lossy("/path/to/.file.tar"), ["tar"]);
        assert_eq!(exts_lossy("/path/to/file.tar.gz"), ["gz", "tar"]);
    }
}
