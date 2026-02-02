use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use clap::Args;
use odyssey::{
    errors::Result,
    utils::{
        constants::{DATABASES_FILE, DATABASES_PATH, SEARCH_PATH},
        search::Search,
    },
};
use serde::{Deserialize, Serialize};

use crate::cli::database::{import::ImportDatabaseArgs, DatabaseKind};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct RemoveDatabaseArgs {
    /// Optional output file
    #[arg(short, long, default_value = "none")]
    version: String,

    kind: DatabaseKind,
}

pub fn remove_database(infos: RemoveDatabaseArgs) -> Result<()> {
    std::fs::create_dir_all(&*DATABASES_PATH)?;

    // Remove database from databases.json file
    let f = File::open(&*DATABASES_FILE)?;
    let reader = BufReader::new(&f);
    let mut databases: Vec<ImportDatabaseArgs> = serde_json::from_reader(reader)?;
    let f = File::create(&*DATABASES_FILE)?;
    if let Some(index) = databases
        .iter()
        .position(|d| d.kind == infos.kind && d.mersion == infos.version)
    {
        databases.remove(index);
    } else {
        println!("No database found!");
        return Ok(());
    }
    let mut writer = BufWriter::new(f);
    if databases.is_empty() {
        write!(&mut writer, "[]")?;
    } else {
        serde_json::to_writer_pretty(&mut writer, &databases)?;
    }
    writer.flush()?;

    // Delete cache
    let name = format!("{:?} {}", infos.kind, infos.version);
    let cache_path = &*DATABASES_PATH.join(&name);
    std::fs::remove_file(cache_path)?;

    // Delete search index
    std::fs::create_dir_all(&*SEARCH_PATH)?;
    let mut search = Search::new()?;
    search.delete_database(&name)?;
    Ok(())
}
