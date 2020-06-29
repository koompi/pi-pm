#![allow(unused_imports, unused_assignments)]
pub mod config;
pub mod database;
pub mod helpers;
pub mod interfaces;
pub mod operations;
pub mod schemas;

use database::{db, sync};

fn main() {
    sync::sync();
    let app_stream = db::init("root/store/db/store.json");
    let app_registry = db::init("root/store/db/installed.json");
    // println!("{:#?}", app_stream);
    // println!("{:#?}", app_registry);
    app_registry.install(app_stream, vec!["nodejs", "atom", "code"])
    // println!("{:#?}", local_db);
    // println!("{:?}", local_db.search_rbool("code"));
    // println!("{:?}", local_db.search_rindex("nodejs"));
    // local_db.search_papp("nodejs");
    // let app = local_db.search_rapp("nodejs");
    // println!("{:#?}", app.unwrap());
}
