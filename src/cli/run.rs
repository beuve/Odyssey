use std::path::PathBuf;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use clap::Args;
use odyssey::comput::impacts::ImpactCategory;
use odyssey::utils::search::Search;
use odyssey::{
    comput::lca::Database, errors::Result, parsers::load_database, utils::matrix::MappedVector,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct RunCommand {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseInfos {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exchange {
    database: Option<DatabaseInfos>,
    file: Option<String>,
    location: Option<String>,
    unit: Option<String>,
    name: Option<String>,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    exchanges: Vec<Exchange>,
}

fn import_from_database(
    database_infos: &DatabaseInfos,
    databases: &mut HashMap<String, Box<dyn Database>>,
    rfs: &mut HashMap<String, MappedVector<String>>,
    search: &Search,
    exchange: &Exchange,
    amount: f64,
) -> Result<()> {
    let database_name = format!("{}_{}", database_infos.name, database_infos.version);
    let exchange_name = exchange.name.clone().unwrap();
    let id = search.search_for_ids(
        &exchange_name,
        Some(&database_name),
        exchange.location.as_deref(),
        exchange.unit.as_deref(),
    )?;
    match &id[..] {
        [] => panic!("No matching activity for {}", exchange_name),
        [a] => {
            let database = databases
                .entry(database_name.clone())
                .or_insert(load_database(
                    &database_infos.name,
                    &database_infos.version,
                )?);

            let local_rf = rfs
                .entry(database_name)
                .or_insert(database.empty_reference_flow());

            local_rf.set(a.clone(), amount * exchange.amount).unwrap();
        }
        _ => panic!("Multiple matching activities for {}", exchange_name),
    }
    Ok(())
}

pub fn import_from_file(
    path: &Path,
    databases: &mut HashMap<String, Box<dyn Database>>,
    rfs: &mut HashMap<String, MappedVector<String>>,
    amount: f64,
) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    let activity: Activity = serde_json::from_reader(reader)?;

    let search = Search::new()?;

    for e in activity.exchanges {
        match (&e.database, &e.file) {
            (Some(database_infos), None) => {
                import_from_database(database_infos, databases, rfs, &search, &e, amount)?
            }
            (None, Some(file)) => import_from_file(Path::new(file), databases, rfs, amount * e.amount)?,
            (Some(_), Some(_)) => panic!(
                "Both database and file were provided for exchange {:?} in file {:?}",
                e.name, path
            ),
            (None, None) => panic!("No data source (either database or file) was provided for exchange {:?} in file {:?}", e.name, path),
        }
    }
    Ok(())
}

pub fn run_lca(path: &Path) -> Result<()> {
    let mut databases: HashMap<String, Box<dyn Database>> = HashMap::new();
    let mut rfs: HashMap<String, MappedVector<String>> = HashMap::new();

    import_from_file(path, &mut databases, &mut rfs, 1f64)?;

    let mut res = ImpactCategory::get_empty_vector();
    for (db, rf) in rfs.iter() {
        res += databases.get_mut(db).unwrap().lca(rf)?;
    }

    for i in 0..res.values.len() {
        if let Some(ImpactCategory::EF31(v)) = res.mapping.get_by_right(&i) {
            println!("{:width$?}: {:.4e}", v, res.values[i], width = 20)
        }
    }
    Ok(())
}
