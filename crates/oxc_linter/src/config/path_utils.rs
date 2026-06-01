use std::path::{Component, Path, PathBuf};

/// Normalize a path by removing `.` and resolving `..` components without touching the filesystem.
#[must_use]
pub fn normalize_lexical_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut result = PathBuf::new();

    for component in path.as_ref().components() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            Component::Normal(c) => {
                result.push(c);
            }
            Component::RootDir | Component::Prefix(prefix) => {
                result.push(prefix.as_os_str());
            }
        }
    }

    result
}

/// Resolve `path` to an absolute path using `base` when it is relative, then normalize lexically.
#[must_use]
pub fn resolve_absolute_path(base: &Path, path: &Path) -> PathBuf {
    let joined = if path.is_absolute() { path.to_path_buf() } else { base.join(path) };
    let absolute = std::path::absolute(&joined).unwrap_or(joined);
    normalize_lexical_path(absolute)
}

/// Return `true` if `prefix` is `path` or an ancestor directory of `path`.
#[must_use]
pub fn is_path_prefix(prefix: &Path, path: &Path) -> bool {
    let prefix = normalize_lexical_path(prefix);
    let path = normalize_lexical_path(path);

    if prefix == path {
        return true;
    }

    let mut prefix_components = prefix.components();
    for component in path.components() {
        match prefix_components.next() {
            None => return true,
            Some(prefix_component) if prefix_component == component => {}
            Some(_) => return false,
        }
    }

    false
}

/// Return `true` if two paths refer to the same location after lexical normalization.
#[must_use]
pub fn paths_equal(a: &Path, b: &Path) -> bool {
    normalize_lexical_path(a) == normalize_lexical_path(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_lexical_path() {
        assert_eq!(
            normalize_lexical_path("/root/directory/./.oxlintrc.json"),
            PathBuf::from("/root/directory/.oxlintrc.json")
        );
    }

    #[test]
    fn test_is_path_prefix() {
        assert!(is_path_prefix(
            Path::new("/repo/packages/foo"),
            Path::new("/repo/packages/foo/bar.ts")
        ));
        assert!(!is_path_prefix(
            Path::new("/repo/packages/foo"),
            Path::new("/repo/packages/food/bar.ts")
        ));
    }

    #[test]
    fn test_paths_equal() {
        assert!(paths_equal(
            Path::new("/repo/./.oxlintrc.json"),
            Path::new("/repo/.oxlintrc.json")
        ));
    }
}
