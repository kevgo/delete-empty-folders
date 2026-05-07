use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Last path segments of directories this tool never recurses into (no listing inside, no deletion there).
const SKIP_DIR_NAMES: &[&str] = &[".git", "node_modules"];

fn skip_directory(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|name| SKIP_DIR_NAMES.contains(&name))
}

fn main() -> io::Result<()> {
    let root = env::current_dir()?;
    remove_empty_descendants(&root, &root)?;
    Ok(())
}

/// Deletes empty directories under `dir`, depth-first. Never removes `root` itself.
fn remove_empty_descendants(dir: &Path, root: &Path) -> io::Result<()> {
    let entries = fs::read_dir(dir)?;
    let mut has_children = false;
    for entry in entries {
        has_children = true;
        let entry = entry?;
        let Ok(filetype) = entry.file_type() else {
            continue;
        };
        if !filetype.is_dir() {
            continue;
        }
        let path = entry.path();
        if skip_directory(&path) {
            continue;
        }
        remove_empty_descendants(&path, root)?;
    }

    if dir == root || has_children {
        return Ok(());
    }

    let relative_path = dir.strip_prefix(root).unwrap_or(dir);
    println!("removing empty directory: {}", relative_path.display());
    fs::remove_dir(dir)
}
