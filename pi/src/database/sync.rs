use crate::{
    config::config,
    helpers::{
        compare::compare,
        download::download,
        file::{file_reader, file_writer},
        REST::get,
    },
    schemas::{config::Config, store::Store, version::Version},
};
use colored::Colorize;
use std::process::Command;

fn version(url: &str) -> Version {
    let v_string = get(url);
    let v: Version = serde_json::from_str(&String::from(v_string)).expect("Error reading version");

    v
}

pub fn sync() {
    let configurations: Config = config::get_repos();
    if !configurations.production {
        println!(
            "{}\n{}",
            "WARNING:".yellow().bold(),
            "This software is running development mode.".yellow()
        );
    };
    let repo_address: String = String::from(configurations.repos[0].address.clone());
    let db_address = format!("{}/db/db.json", &repo_address);
    let version_address = format!("{}/db/version", &repo_address);
    let server_version: Version = version(&version_address);
    let local_version: Version = Version { version: 20200617 };
    let cmp_result: i8 = compare(local_version.version, server_version.version);
    println!("{}", "AppStore is checking for database updates...".blue());
    if cmp_result != 0 {
        println!("Downloading database updates...");
        download(&db_address, "root/store/db", "store.json", false);
    } else {
        println!("{}", "You are running the latest software version.".green());
    };
}
