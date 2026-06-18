use ignore::gitignore::Gitignore;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

/// Last path segments of directories this tool never recurses into (no listing inside, no deletion there).
pub const SKIP_DIR_NAMES: &[&str] = &[".git"];

/// treat these directories as non-existent
pub const IGNORE_NAMES: &[&str] = &["__pycache__"];

fn main() -> io::Result<()> {
    // skip the path to the executable
    if let Some(flag) = env::args().nth(1) {
        if flag == "--help" || flag == "-h" {
            print_help();
            return Ok(());
        }
        if flag == "--version" || flag == "-V" {
            print_version();
            return Ok(());
        }
    }
    let cwd = env::current_dir()?;
    let gitignore_path = cwd.join(".gitignore");
    let gitignore_file = if gitignore_path.is_file() {
        let (gitignore, err) = Gitignore::new(&gitignore_path);
        if let Some(e) = err {
            return Err(io::Error::other(e));
        }
        Some(gitignore)
    } else {
        None
    };
    remove_empty_descendants(&cwd, &cwd, gitignore_file.as_ref())?;
    Ok(())
}

/// Deletes empty directories under `dir`, depth-first. Never removes `root` itself.
/// Returns true if the directory was removed.
///
/// # Errors
///
/// Returns [`io::Error`] when any file operation fails.
pub fn remove_empty_descendants(
    dir: &Path,
    root: &Path,
    gitignore: Option<&Gitignore>,
) -> io::Result<bool> {
    let entries = fs::read_dir(dir)?;
    let mut has_children = false;
    let mut child_dirs = Vec::new();
    let ignore_names: Vec<&OsStr> = IGNORE_NAMES.iter().map(OsStr::new).collect();
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
        if let Some(file_name) = path.file_name()
            && ignore_names.contains(&file_name)
        {
            continue;
        }
        if skip_directory(&path, gitignore) {
            has_children = true;
            continue;
        }
        child_dirs.push(path);
    }
    // Note: sort the child directories because certain file systems return entries in random order.
    child_dirs.sort();
    for path in child_dirs {
        let gone_now = remove_empty_descendants(&path, root, gitignore)?;
        if !gone_now {
            has_children = true;
        }
    }
    if dir == root || has_children {
        return Ok(false);
    }
    let relative_path = dir.strip_prefix(root).unwrap_or(dir);
    println!("removing empty directory: {}", relative_path.display());
    fs::remove_dir_all(dir)?;
    Ok(true)
}

/// indicates whether the given directory should be skipped
fn skip_directory(path: &Path, gitignore: Option<&Gitignore>) -> bool {
    let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    if SKIP_DIR_NAMES.contains(&file_name) {
        return true;
    }
    let Some(gitignore) = gitignore else {
        return false;
    };
    gitignore.matched(path, true).is_ignore()
}

fn print_help() {
    println!(
        r"Deletes all empty directories in the current directory and its subdirectories.

Usage: delete-empty-folders"
    );
}

fn print_version() {
    println!("delete-empty-folders {}", env!("CARGO_PKG_VERSION"));
}
