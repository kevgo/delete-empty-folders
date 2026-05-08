use cucumber::gherkin::Step;
use cucumber::{World, given, then, when};
use rand::Rng;
use std::borrow::Cow;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::{ExitStatus, Output};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, str};
use tokio::fs;
use tokio::process::Command;

#[derive(Debug, World)]
#[world(init = Self::new)]
struct DeleteWorld {
    /// the workspace for the current scenario
    dir: PathBuf,

    /// files and folders in the workspace before running the executable
    initial_contents: Vec<FSEntry>,

    /// the result of the running executable
    output: Option<Output>,
}

/// a file or folder in the workspace
#[derive(Debug, PartialEq)]
enum FSEntry {
    File(PathBuf),
    Folder(PathBuf),
}

impl DeleteWorld {
    fn new() -> Self {
        Self {
            dir: tmp_dir(),
            initial_contents: vec![],
            output: None,
        }
    }

    /// provides the exit code of the last run
    fn exit_status(&self) -> ExitStatus {
        match &self.output {
            Some(output) => output.status,
            None => panic!(),
        }
    }

    /// provides the textual output of the Atlanta run
    fn output(&self) -> Cow<'_, str> {
        match &self.output {
            Some(output) => String::from_utf8_lossy(&output.stdout),
            None => Default::default(),
        }
    }
}

#[given("a folder with contents:")]
async fn a_folder_with(world: &mut DeleteWorld, step: &Step) {
    let table = step.table.as_ref().unwrap();
    for row in &table.rows {
        let path = world.dir.join(&row[1]);
        match row[0].as_str() {
            "FILE" => {
                let parent = path.parent().unwrap();
                fs::create_dir_all(parent).await.unwrap();
                fs::write(&path, "x".as_bytes()).await.unwrap();
            }
            "FOLDER" => fs::create_dir_all(path).await.unwrap(),
            other => panic!("unexpected entry type: {}", other),
        };
    }
}

#[given("a .gitignore file with contents:")]
async fn gitignore_file(world: &mut DeleteWorld, step: &Step) {
    let path = world.dir.join(".gitignore");
    let content = step.docstring.as_ref().unwrap();
    fs::write(&path, content.as_bytes()).await.unwrap();
}

/// path to the `delete-empty-folders` binary built by Cargo (`target/debug/…`)
fn delete_empty_folders_executable() -> PathBuf {
    env::current_dir()
        .expect("cannot determine the current directory")
        .join("target")
        .join("debug")
        .join(format!("delete-empty-folders{}", env::consts::EXE_SUFFIX))
}

#[when(expr = "running delete-empty-folders")]
async fn running(world: &mut DeleteWorld) {
    load_dir_contents(&world.dir, &mut world.initial_contents).await;
    world.output = Some(
        Command::new(delete_empty_folders_executable())
            .current_dir(&world.dir)
            .output()
            .await
            .expect("cannot find the 'delete-empty-folders' executable"),
    );
    assert!(world.exit_status().success());
}

#[then("it prints:")]
fn it_prints(world: &mut DeleteWorld, step: &Step) {
    let want = step.docstring.as_ref().unwrap();
    let have = world.output().replace("\\", "/");
    pretty::assert_eq!(have.trim(), want.trim());
}

#[then("it prints nothing")]
fn it_prints_nothing(world: &mut DeleteWorld) {
    let have = world.output();
    assert_eq!(have.trim(), "");
}

#[then(expr = "the workspace is empty")]
async fn workspace_is_empty(world: &mut DeleteWorld) {
    let mut entries = fs::read_dir(&world.dir).await.unwrap();
    assert!(entries.next_entry().await.unwrap().is_none());
}

#[then(expr = "the workspace is unchanged")]
async fn workspace_is_unchanged(world: &mut DeleteWorld) {
    let mut entries = vec![];
    load_dir_contents(&world.dir, &mut entries).await;
    assert_eq!(entries, world.initial_contents);
}

#[then(expr = "the workspace contains:")]
async fn workspace_contains(world: &mut DeleteWorld, step: &Step) {
    let mut want = vec![];
    let table = step.table.as_ref().unwrap();
    for row in &table.rows {
        let path = world.dir.join(&row[1]);
        let entry = match row[0].as_str() {
            "FILE" => FSEntry::File(path),
            "FOLDER" => FSEntry::Folder(path),
            other => panic!("unexpected entry type: {}", other),
        };
        want.push(entry);
    }
    let mut have = vec![];
    load_dir_contents(&world.dir, &mut have).await;
    assert_eq!(have, want);
}

/// creates a temporary directory
fn tmp_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(3)
        .map(char::from)
        .collect();
    let cwd = env::current_dir().expect("cannot determine the current directory");
    let dir = cwd.join("tmp").join(format!("{}-{}", timestamp, rand));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// provides the contents of the given directory and all its subdirectories
fn load_dir_contents<'a>(
    dir: &'a Path,
    result: &'a mut Vec<FSEntry>,
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await.unwrap();
        while let Some(entry) = entries.next_entry().await.unwrap() {
            let file_type = entry.file_type().await.unwrap();
            if file_type.is_dir() {
                result.push(FSEntry::Folder(entry.path()));
                load_dir_contents(&entry.path(), result).await;
            } else if file_type.is_file() {
                result.push(FSEntry::File(entry.path()));
            } else {
                panic!("unexpected file type: {:?}", file_type);
            }
        }
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    DeleteWorld::run("features").await;
}
