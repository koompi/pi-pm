use crate::{
    config::config,
    helpers::{compare::compare, download::download, file::vserion_reader, REST::get},
    schemas::{config::Config, version::Version},
};
use colored::Colorize;

fn cloud_version(url: &str) -> Version {
    let v_string = get(url);
    let v: Version = serde_json::from_str(&String::from(v_string)).expect("Error reading version");

    v
}

fn disk_version(path: &str) -> Version {
    vserion_reader(path)
}

pub fn sync() {
    let configurations: Config = config::get();
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
    let server_version: Version = cloud_version(&version_address);
    let local_version: Version = disk_version(&config::get().version_file);
    let cmp_result: i8 = compare(local_version.version, server_version.version);
    println!("{}", "AppStore is checking for database updates...".blue());
    if cmp_result != 0 {
        println!("Downloading database updates...");
        download(&db_address, "root/store/db", "store.json", false);
    } else {
        println!("{}", "AppStore database is up to date.".green());
    };
}
