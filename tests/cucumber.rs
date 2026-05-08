use cucumber::gherkin::Step;
use cucumber::{World, given, then, when};
use rand::Rng;
use std::borrow::Cow;
use std::path::PathBuf;
use std::process::{ExitStatus, Output};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, str};
use tokio::process::Command;
use tokio::{fs, io};

#[derive(Debug, World)]
#[world(init = Self::new)]
struct DeleteWorld {
    /// the directory containing the test files of the current scenario
    dir: PathBuf,

    /// the result of running the executable
    output: Option<Output>,
}

impl DeleteWorld {
    fn new() -> Self {
        Self {
            dir: tmp_dir(),
            output: None,
        }
    }
}

impl DeleteWorld {
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

#[given(expr = "a file {string}")]
async fn a_file(world: &mut DeleteWorld, filename: String) -> io::Result<()> {
    let filepath = world.dir.join(filename);
    fs::create_dir_all(filepath.parent().unwrap())
        .await
        .unwrap();
    fs::write(&filepath, "x".as_bytes()).await
}

#[given(expr = "a folder {string}")]
async fn a_folder(world: &mut DeleteWorld, name: String) -> io::Result<()> {
    let folder_path = &world.dir.join(name);
    fs::create_dir_all(folder_path).await
}

#[when(expr = "running delete-empty-folders")]
async fn running(world: &mut DeleteWorld) {
    let cmd = "../../target/debug/delete-empty-folders";
    world.output = Some(
        Command::new(cmd)
            .current_dir(&world.dir)
            .output()
            .await
            .expect("cannot find the 'delete-empty-folders' executable"),
    );
    assert!(world.exit_status().success());
}

#[then("it prints:")]
fn it_prints(world: &mut DeleteWorld, step: &Step) {
    let want = step.docstring.as_ref().unwrap().trim();
    let have = world.output();
    pretty::assert_eq!(have.trim(), want);
}

#[then("it prints nothing")]
fn it_prints_nothing(world: &mut DeleteWorld) {
    let have = world.output();
    pretty::assert_eq!(have.trim(), "");
}

#[then(expr = "the workspace is empty")]
async fn workspace_is_empty(world: &mut DeleteWorld) {
    let mut entries = fs::read_dir(&world.dir).await.unwrap();
    assert!(entries.next_entry().await.unwrap().is_none());
}

#[then(expr = "the workspace contains a folder {string}")]
fn contains_folder(world: &mut DeleteWorld, folder: String) {
    assert!(world.dir.join(folder).is_dir())
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    DeleteWorld::run("features").await;
}
