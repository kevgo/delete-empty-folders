use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Last path segments of directories this tool never recurses into (no listing inside, no deletion there).
pub const SKIP_DIR_NAMES: &[&str] = &[".git"];

fn main() -> io::Result<()> {
    let cwd = env::current_dir()?;
    let gitignore_path = cwd.join(".gitignore");
    let gitignore_file = if gitignore_path.is_file() {
        Some(gitignore::File::new(&gitignore_path).map_err(io::Error::other)?)
    } else {
        None
    };
    remove_empty_descendants(&cwd, &cwd, gitignore_file.as_ref())?;
    Ok(())
}

type Gitignore<'a> = Option<&'a gitignore::File<'a>>;

/// Deletes empty directories under `dir`, depth-first. Never removes `root` itself.
/// Returns true if the directory was removed.
///
/// # Errors
///
/// Returns [`io::Error`] when any file operation fails.
pub fn remove_empty_descendants(
    dir: &Path,
    root: &Path,
    gitignore_file: Gitignore,
) -> io::Result<bool> {
    let entries = fs::read_dir(dir)?;
    let mut has_children = false;
    let mut child_dirs = Vec::new();
    for entry in entries {
        let entry = entry?;
        let Ok(filetype) = entry.file_type() else {
            has_children = true;
            continue;
        };
        if !filetype.is_dir() {
            has_children = true;
            continue;
        }
        let path = entry.path();
        if skip_directory(&path, gitignore_file) {
            has_children = true;
            continue;
        }
        child_dirs.push(path);
    }
    // Note: sort the child directories because certain file systems return entries in random order.
    child_dirs.sort();
    for path in child_dirs {
        let gone_now = remove_empty_descendants(&path, root, gitignore_file)?;
        if !gone_now {
            has_children = true;
        }
    }
    if dir == root || has_children {
        return Ok(false);
    }
    let relative_path = dir.strip_prefix(root).unwrap_or(dir);
    println!("removing empty directory: {}", relative_path.display());
    fs::remove_dir(dir)?;
    Ok(true)
}

/// indicates whether the given directory should be skipped
fn skip_directory(path: &Path, gitignore_file: Gitignore) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    if SKIP_DIR_NAMES.contains(&name) {
        return true;
    }
    let Some(file) = gitignore_file else {
        return false;
    };
    let Ok(is_excluded) = file.is_excluded(path) else {
        return false;
    };
    is_excluded
}
