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

    let mut has_remaining = false;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                eprintln!("skip (permission denied): {}", dir.display());
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        let path = entry.path();
        let filetype = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => {
                has_remaining = true;
                continue;
            }
        };

        if filetype.is_dir() {
            remove_empty_descendants(&path, root)?;
            if path.exists() {
                has_remaining = true;
            }
        } else {
            has_remaining = true;
        }
    }

    if dir != root && !has_remaining {
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
