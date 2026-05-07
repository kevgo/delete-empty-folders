use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let root = env::current_dir()?;
    remove_empty_descendants(&root, &root)?;
    Ok(())
}

/// Deletes empty directories under `root`, depth-first. Never removes `root` itself.
fn remove_empty_descendants(dir: &Path, root: &Path) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!("skip (permission denied): {}", dir.display());
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                eprintln!("skip (permission denied): {}", dir.display());
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        let filetype = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if filetype.is_dir() {
            remove_empty_descendants(&entry.path(), root)?;
        }
    }

    if dir != root && is_dir_empty(dir)? {
        match fs::remove_dir(dir) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                eprintln!("skip (permission denied): {}", dir.display());
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn is_dir_empty(dir: &Path) -> io::Result<bool> {
    Ok(fs::read_dir(dir)?.next().is_none())
}
