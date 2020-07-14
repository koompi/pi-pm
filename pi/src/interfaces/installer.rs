use crate::{
    helpers::{download::download, extract::extract, file::file_writer},
    schemas::store::Store,
};
use colored::Colorize;
use itertools::Itertools;

fn _flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

impl Store {
    // installing methods
    // FLOW
    // 1. search from db if app exist
    // 2. search if app already installed
    // 3. resolve packages to be downloaded
    // 4. download packages
    // 5. verify package signature
    // 6. resolve install order
    // 7. install packages
    // 8. update installed database
    // 9. commit transaction
    pub fn install(&mut self, db: &Store, mut apps: Vec<&str>) {
        // sort the app order so it reduce the loop cycle
        apps.sort();
        // find app from app_stream
        let mut apps_not_found: Vec<&str> = vec![];
        let mut apps_location: Vec<u32> = vec![];
        for app in apps.iter() {
            let res = db.search_rindex(app);
            if res == u32::max_value() {
                apps_not_found.push(app);
            } else {
                apps_location.push(res);
            }
        }

        if apps_not_found.len() > 0 {
            println!(
                "{}\n- {}\n\n{}\n{}",
                "These apps are not found:".red().bold(),
                apps_not_found.join(" \n- ").bold().blink(),
                "The problem can either be:\n- misspelling or\n- those apps are not available in our palform for now.\n\nFeel free to reach out for help at:",
                "https://t.me/koompi".blue()
            );
        } else {
            let mut dep_list: Vec<String> = vec![];
            for app in apps_location.iter() {
                // println!("{}", &db.tar.gzs[*app as usize].name);
                let app_deps = &db.apps[*app as usize].runtime_deps;
                if app_deps.len() > 0 {
                    if app_deps[0] != "none" {
                        for dep in app_deps.iter() {
                            dep_list.push(dep.clone());
                        }
                    };
                };
            }
            dep_list.sort();
            // remove duplicats from dependencies list
            let clean_dep_list: Vec<_> = dep_list.into_iter().unique().collect();

            for dep in clean_dep_list.iter() {
                let res = db.search_rindex(dep);
                if res == u32::max_value() {
                    apps_not_found.push(dep);
                } else {
                    apps_location.push(res);
                }
            }
            // remove dupilcats from app list
            let clean_app_list: Vec<_> = apps_location.into_iter().unique().collect();

            for app in clean_app_list.iter() {
                let src = &db.apps[*app as usize].tarball_src;
                let name = format!("{}.tar.gz", &db.apps[*app as usize].name);
                let dest = "root/store/cache";
                let resume = true;
                if download(src, dest, &name, resume) {
                    println!("[done] {}", name);
                }
            }

            for app in clean_app_list.iter() {
                let path = "root/store/cache";
                let name = &format!("{}.tar.gz", &db.apps[*app as usize].name);
                let dest = "root/";
                extract(path, name, dest).unwrap_or(());
                &self.apps.push(db.apps[*app as usize].clone());
            }
            file_writer(self.clone(), "root/store/db/installed.json").unwrap_or(());
            // TODO:
            // [] check package signature
            // [] resolve installation order
        }
    }
}
