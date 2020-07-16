use crate::{
    helpers::file::file_writer,
    schemas::{app::App, store::Store},
};
use colored::Colorize;
use std::fs::{metadata, remove_dir_all, remove_file, File};

impl Store {
    // pub fn search_existed_locally(&self, name: &str) -> bool {
    //     let mut found: bool = false;
    //     for app in self.apps.iter() {
    //         if app.name == name {
    //             found = true;
    //         }
    //     }
    //     found
    // }

    pub fn get_remove_target(&mut self, apps: Vec<&str>) -> Vec<String> {
        let mut target: Vec<String> = Vec::new();
        for app in apps.iter() {
            for package in self.apps.iter() {
                if app.to_string() == package.name.to_string() {
                    for t in package.content.iter() {
                        target.push(t.to_string());
                    }
                }
            }
        }
        target
    }

    pub fn is_removeable(&mut self, apps: Vec<&str>) -> bool {
        let mut allow: bool = false;
        for app in apps.iter() {
            for p in self.apps.iter() {
                if app.to_string() == p.name {
                    if p.required_by.len() > 0 {
                        if p.required_by[0].to_lowercase() != "none" {
                            let mut response: Vec<String> = Vec::new();

                            for dep in p.required_by.iter() {
                                let result = self.search_rbool(dep);
                                if result {
                                    response.push(dep.to_string());
                                }
                            }

                            if response.len() > 0 {
                                println!(
                                    "NOT ALLOWED! Removing {} may break dependency of {:?}",
                                    app.to_string().to_uppercase(),
                                    response
                                );
                                allow = false;
                            } else {
                                allow = true;
                            }
                        } else {
                            allow = true;
                        }
                    }
                }
            }
        }
        allow
    }

    pub fn remove(&mut self, apps: Vec<&str>) {
        let mut not_found: Vec<String> = Vec::new();

        for app in apps.iter() {
            if !self.search_rbool(app) {
                not_found.push(app.to_string());
            }
        }

        if not_found.len() > 0 {
            println!("These apps are not found: {:?}", not_found);
        } else {
            if self.is_removeable(apps.clone()) {
                let targets = self.get_remove_target(apps.clone());

                for t in targets.iter() {
                    match metadata(t) {
                        Ok(data) => {
                            if data.is_dir() {
                                match remove_dir_all(t) {
                                    Ok(()) => println!("removed: {}", t),
                                    Err(err) => {
                                        println!("Error: {}", err);
                                    }
                                }
                            } else {
                                match remove_file(t) {
                                    Ok(()) => println!("removed: {}", t),
                                    Err(err) => {
                                        println!("Error: {}", err);
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                        }
                    }
                }
                for app in apps.iter() {
                    let index = self.apps.iter().position(|x| *x.name == app.to_string());
                    match index {
                        Some(data) => {
                            self.apps.remove(data);
                        }
                        None => println!("{} :Not found", app),
                    }
                }
                match file_writer(self.clone(), "root/store/db/installed.json") {
                    Ok(()) => println!("Uninstallation completed successfully."),
                    Err(err) => println!("Uninstallation completed with some error:\n{}", err),
                }
            }
        }
    }
}
