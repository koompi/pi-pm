#![allow(unused_imports, unused_assignments, unused_variables)]
pub mod config;
pub mod database;
pub mod helpers;
pub mod interfaces;
pub mod operations;
pub mod schemas;

use config::config::get;
use database::{db, sync};
use schemas::config::Config;
fn main() {
    let configuration: Config = get();
    sync::sync();
    let mut local_db = db::init(&configuration.local_db);
    let mut registry = db::init(&configuration.registry);

    // println!("{:#?}", local_db);
    // println!("{:#?}", registry);
    // registry.install(&local_db, vec!["code", "atom"]);
    // println!("{:#?}", registry);
    // registry.remove(vec!["nodejs"]);
    registry.update(&mut local_db);
    // println!("{:#?}", local_db);
    // println!("{:?}", local_db.search_rbool("code"));
    // println!("{:?}", local_db.search_rindex("nodejs"));
    // local_db.search_papp("nodejs");
    // let app = local_db.search_rapp("nodejs");
    // println!("{:#?}", app.unwrap());
}
