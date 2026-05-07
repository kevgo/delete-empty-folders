use std::env;
use std::io;

use delete_empty_folders::remove_empty_descendants;

fn main() -> io::Result<()> {
    let root = env::current_dir()?;
    remove_empty_descendants(&root, &root)?;
    Ok(())
}
