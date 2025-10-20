use std::{io, path::Path};

use clap::Args;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CreateCommand {
    pub path: String,
}

/// Create a project folder in the current folder
/// with the given [name].
pub fn create_project_folder(name: &str) -> io::Result<()> {
    let path = std::env::current_dir()?.join(name);
    std::fs::create_dir_all(&path)?;
    let cache = path.join(".cache");
    std::fs::create_dir(&cache)?;
    std::fs::create_dir(cache.join("search"))?;
    create_database_file(&path.join("databases.json"))?;
    Ok(())
}

pub fn create_database_file(path: &Path) -> io::Result<()> {
    std::fs::write(path, "[]")?;
    Ok(())
}
