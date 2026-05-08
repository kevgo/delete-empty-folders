use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Last path segments of directories this tool never recurses into (no listing inside, no deletion there).
pub const SKIP_DIR_NAMES: &[&str] = &[".git", "node_modules"];

fn main() -> io::Result<()> {
    let cwd = env::current_dir()?;
    remove_empty_descendants(&cwd, &cwd)?;
    Ok(())
}

/// Deletes empty directories under `dir`, depth-first. Never removes `root` itself.
/// Returns true if the directory was removed.
pub fn remove_empty_descendants(dir: &Path, root: &Path) -> io::Result<bool> {
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
        if skip_directory(&path) {
            has_children = true;
            continue;
        }
        child_dirs.push(path);
    }
    // Note: sort the child directories because certain file systems return entries in random order.
    child_dirs.sort();
    for path in child_dirs {
        let gone_now = remove_empty_descendants(&path, root)?;
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

fn skip_directory(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| SKIP_DIR_NAMES.contains(&name))
}
