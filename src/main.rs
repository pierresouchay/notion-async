use std::str::FromStr;

use clap::Parser;
use notion::ids::{DatabaseId, AsIdentifier};
use notion::models::Database;
use notion::{NotionApi};
use tokio;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Pierre Souchay")]
struct Opts {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Sync
    Sync {
        database_id: String,
    }
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let notion_token = match std::env::var("NOTION_TOKEN") {
        Ok(token) => token,
        _ => panic!("$NOTION_TOKEN not set")
    };
    let notion_api = NotionApi::new(
        notion_token
    ).unwrap();
    let _res = match opts.command {
        SubCommand::Sync { database_id} => list_tasks(&notion_api, database_id),
    }.await;
}

async fn list_tasks(notion_api: &NotionApi, database_id: String){
    let db_id = DatabaseId::from_str(&database_id).unwrap();
    let db_req = notion_api.get_database(db_id).await;
    match db_req {
        Ok(db) => println!("{}: {}: {:#?}", db.as_id(), db.last_edited_time, db.properties.values()),
        _ => {}
    }
}