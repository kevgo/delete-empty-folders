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
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!("cannot list (permission denied): {}", dir.display());
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    let mut contains_files = false;

    for entry in entries {
        contains_files = true;
        let entry = match entry {
            Ok(e) => e,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                eprintln!("cannot read (permission denied): {}", dir.display());
                continue;
            }
            Err(e) => return Err(e),
        };
        let path = entry.path();
        let Ok(filetype) = entry.file_type() else {
            continue;
        };
        if filetype.is_dir() {
            if skip_directory(&path) {
                continue;
            }
            remove_empty_descendants(&path, root)?;
        }
    }

    println!("dir: {} {}", dir.display(), contains_files);
    if dir != root && !contains_files {
        match fs::remove_dir(dir) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                eprintln!("cannot remove (permission denied): {}", dir.display());
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
