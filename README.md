# path-utils

Utility library for `std::path::Path` and `std::path::PathBuf`.

```rust
use std::path::Path;
use path_utils::PathExt;

let path = Path::new("file.tar.gz");
let extensions = path.extensions_lossy().collect::<Vec<String>>();
assert_eq!(extensions, ["gz", "tar"]);
```
