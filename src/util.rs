use std::path::PathBuf;

pub fn extensions_lossy(path: impl Into<PathBuf>) -> Vec<String> {
    Extensions::new(path).collect()
}

pub struct Extensions {
    path: PathBuf,
}

impl Extensions {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl Iterator for Extensions {
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

    #[test]
    fn extensions() {
        assert!(extensions_lossy(PathBuf::from("/path/to/file")).is_empty());
        assert!(extensions_lossy(PathBuf::from("/path/to/.file")).is_empty());

        assert_eq!(
            extensions_lossy(PathBuf::from("/path/to/file.tar")),
            ["tar"]
        );

        assert_eq!(
            extensions_lossy(PathBuf::from("/path/to/.file.tar")),
            ["tar"]
        );

        assert_eq!(
            extensions_lossy(PathBuf::from("/path/to/file.tar.gz")),
            ["gz", "tar"],
        );
    }
}
