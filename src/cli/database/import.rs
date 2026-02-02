use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use clap::Args;
use console::style;
use indicatif::ProgressBar;
use odyssey::{
    errors::Result,
    parsers::ecospold2::Ecoinvent,
    utils::{
        constants::{DATABASES_FILE, DATABASES_PATH, SEARCH_PATH},
        search::Search,
    },
};
use serde::{Deserialize, Serialize};

use crate::cli::database::DatabaseKind;

#[derive(Debug, Args, Serialize, Deserialize, Clone)]
pub struct ImportDatabaseArgs {
    /// Optional output file
    #[arg(short, long, default_value = "none")]
    pub mersion: String,

    /// Optional output file
    #[arg(short, long)]
    pub path: PathBuf,

    pub kind: DatabaseKind,
}

pub fn import_database(mut infos: ImportDatabaseArgs) -> Result<()> {
    let name = format!("{:?}_{}", infos.kind, infos.mersion);
    std::fs::create_dir_all(&*DATABASES_PATH)?;

    // Register database in databases.json file
    let bar = ProgressBar::new_spinner().with_message("Registering database");
    register_database(&mut infos)?;
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.finish_with_message(format!("{} Registered database", style("✓").green()));

    // Save cache of database in global folder
    let data_path = Path::new(&infos.path);
    let bar = ProgressBar::new_spinner().with_message("Loading database");
    bar.enable_steady_tick(Duration::from_millis(100));
    let cache_path = &*DATABASES_PATH.join(&name);
    let database = match infos.kind {
        DatabaseKind::Ecoinvent => Ecoinvent::load(&infos.mersion, data_path, Some(cache_path))?,
    };
    bar.finish_with_message(format!("{} Loading database", style("✓").green()));

    // Index search
    let bar = ProgressBar::new_spinner().with_message("Indexing database");
    bar.enable_steady_tick(Duration::from_millis(100));
    std::fs::create_dir_all(&*SEARCH_PATH)?;
    let search = Search::new()?;
    search.index_database(&database)?;
    bar.finish_with_message(format!("{} Indexing database", style("✓").green()));
    Ok(())
}

fn register_database(infos: &mut ImportDatabaseArgs) -> Result<()> {
    let mut databases: Vec<ImportDatabaseArgs> = vec![];
    if let Ok(f) = File::open(&*DATABASES_FILE) {
        let reader = BufReader::new(&f);
        databases = serde_json::from_reader(reader)?;
        if databases
            .iter()
            .any(|e| e.kind == infos.kind && e.mersion == infos.mersion)
        {
            return Ok(());
        }
    }
    let f = File::create(&*DATABASES_FILE)?;
    infos.path = std::fs::canonicalize(&infos.path)?;
    databases.push(infos.to_owned());
    let mut writer = BufWriter::new(f);
    serde_json::to_writer_pretty(&mut writer, &databases)?;
    writer.flush()?;
    Ok(())
}
