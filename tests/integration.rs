//! Integration tests: temp workspace, synthetic trees, then verify filesystem results.

use indoc::indoc;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn mkdir(relative: &str, root: &Path) -> PathBuf {
    let path = parse_path(relative, root);
    fs::create_dir_all(&path).unwrap();
    path
}

fn touch(relative: &str, root: &Path) -> PathBuf {
    let path = parse_path(relative, root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, b"x").unwrap();
    path
}

fn parse_path(p: &str, root: &Path) -> PathBuf {
    let full_path = root.join(p);
    PathBuf::from(full_path)
}

fn assert_dir_exists(path: &Path) {
    assert!(path.is_dir(), "expected directory {:?}", path);
}

fn assert_path_absent(path: &Path) {
    assert!(!path.exists(), "expected {:?} to be absent", path);
}

fn workspace() -> TempDir {
    TempDir::new().unwrap()
}

/// Path to the `delete-empty-folders` binary for this build profile.
/// `cargo test` may not set `CARGO_BIN_EXE_*`, so we derive it from `CARGO_MANIFEST_DIR`.
fn cli_executable() -> PathBuf {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let result = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(profile)
        .join("delete-empty-folders");
    assert!(
        result.is_file(),
        "expected CLI binary at {:?}; run `cargo build` or `cargo test` for this package first",
        result
    );
    result
}

fn run_cli(root: &Path) -> io::Result<String> {
    let exe = cli_executable();
    let output = Command::new(&exe).current_dir(root).output()?;
    assert!(output.status.success());
    Ok(String::from_utf8(output.stdout).unwrap())
}

// #[test]
// fn removes_nested_empty_directories() -> io::Result<()> {
//     let root = workspace();
//     let a = root.path().join("a");
//     let b = a.join("b");
//     let c = b.join("c");
//     mkdir(&c);

// remove_empty_descendants(root.path(), root.path())?;

//     assert_path_absent(&a);
//     assert_dir_exists(root.path());
//     Ok(())
// }

// #[test]
// fn keeps_directories_that_contain_files() -> io::Result<()> {
//     let root = workspace();
//     let keep = root.path().join("keep");
//     touch(&keep.join("file.txt"));
//     let empty = root.path().join("empty");
//     mkdir(&empty);

// remove_empty_descendants(root.path(), root.path())?;

//     assert_dir_exists(&keep);
//     assert_path_absent(&empty);
//     Ok(())
// }

// #[test]
// fn keeps_parent_when_subdirectory_had_content() -> io::Result<()> {
//     let root = workspace();
//     let parent = root.path().join("parent");
//     touch(&parent.join("note"));

// remove_empty_descendants(root.path(), root.path())?;

//     assert_dir_exists(&parent);
//     assert!(parent.join("note").is_file());
//     Ok(())
// }

// #[test]
// fn does_not_recurse_into_dot_git() -> io::Result<()> {
//     let root = workspace();
//     let git = root.path().join(".git");
//     mkdir(&git);
//     // If we recursed, an empty inner dir could be removed; we skip .git entirely.
//     assert!(git.read_dir().unwrap().next().is_none());

// remove_empty_descendants(root.path(), root.path())?;

//     assert_dir_exists(&git);
//     Ok(())
// }

// #[test]
// fn does_not_recurse_into_node_modules() -> io::Result<()> {
//     let root = workspace();
//     let nm = root.path().join("node_modules");
//     mkdir(&nm);

// remove_empty_descendants(root.path(), root.path())?;

//     assert_dir_exists(&nm);
//     Ok(())
// }

#[test]
fn binary_deletes_nested_empty_folders() -> io::Result<()> {
    let root = workspace();
    let root_path = root.path();
    let folder_with_empty_subfolder = root_path.join("folder");
    let empty_subfolder = folder_with_empty_subfolder.join("empty_subfolder");

    let output = run_cli(root_path)?;

    assert_eq!(
        output,
        indoc! {"
        removing empty directory: folder/subfolder
        removing empty directory: folder
    "}
    );

    assert_path_absent(&empty_subfolder);
    assert_path_absent(&folder_with_empty_subfolder);
    Ok(())
}

// #[test]
// fn removes_sibling_empty_trees_but_keeps_nonempty_branch() -> io::Result<()> {
//     let root = workspace();
//     // branch1: a/b — all empty → removed
//     mkdir(&root.path().join("branch1").join("a").join("b"));
//     // branch2: p has a file; p/q is empty → q is removed, p stays
//     let p = root.path().join("branch2").join("p");
//     mkdir(&p.join("q"));
//     touch(&p.join("data"));

//     remove_empty_descendants(root.path(), root.path())?;

//     assert_path_absent(&root.path().join("branch1"));
//     assert_dir_exists(&p);
//     assert_path_absent(&p.join("q"));
//     assert!(p.join("data").is_file());
//     Ok(())
// }
