use std::env;
use std::fs;
use std::io;
use std::path::Path;

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
        println!("entry: {}", path.display());
        let Ok(filetype) = entry.file_type() else {
            continue;
        };
        if filetype.is_dir() {
            remove_empty_descendants(&path, root)?;
        }
    }

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
