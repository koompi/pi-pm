#![allow(unused_imports, unused_assignments, unused_variables)]
pub mod config;
pub mod database;
pub mod helpers;
pub mod interfaces;
pub mod operations;
pub mod schemas;

use database::{db, sync};

fn main() {
    sync::sync();
    let local_db = db::init("root/store/db/store.json");
    let mut registry = db::init("root/store/db/installed.json");
    // println!("{:#?}", local_db);
    // println!("{:#?}", registry);
    // registry.install(&local_db, vec!["code", "atom"]);
    // println!("{:#?}", registry);
    registry.remove(vec!["code", "atom"]);
    // println!("{:#?}", local_db);
    // println!("{:?}", local_db.search_rbool("code"));
    // println!("{:?}", local_db.search_rindex("nodejs"));
    // local_db.search_papp("nodejs");
    // let app = local_db.search_rapp("nodejs");
    // println!("{:#?}", app.unwrap());
}
