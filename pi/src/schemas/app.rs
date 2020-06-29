#![allow(non_camel_case_types)]
use super::{architecture::ARCHITECTURE, license::LICENSE};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct App {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub license: LICENSE,
    pub architecture: Vec<ARCHITECTURE>,
    pub version: String,
    pub build_date: i32,
    pub signature: String,
    pub upstream: String,
    pub tarball_src: String,
    pub owner: String,
    pub owner_website: String,
    pub maintainer: Vec<String>,
    pub build_deps: Vec<String>,
    pub runtime_deps: Vec<String>,
    pub optional_deps: Vec<String>,
    pub conflict_with: Vec<String>,
    pub required_by: Vec<String>,
    pub provide: Vec<String>,
    pub content: Vec<String>,
}

impl App {
    pub fn new(
        id: String,
        name: String,
        desc: String,
        license: LICENSE,
        architecture: Vec<ARCHITECTURE>,
        version: String,
        build_date: i32,
        signature: String,
        upstream: String,
        tarball_src: String,
        owner: String,
        owner_website: String,
        maintainer: Vec<String>,
        build_deps: Vec<String>,
        runtime_deps: Vec<String>,
        optional_deps: Vec<String>,
        conflict_with: Vec<String>,
        required_by: Vec<String>,
        provide: Vec<String>,
        content: Vec<String>,
    ) -> App {
        App {
            id: id.to_string(),
            name: name.to_string(),
            desc: desc.to_string(),
            license: license,
            architecture: architecture,
            version: version.to_string(),
            build_date: build_date as i32,
            signature: signature.to_string(),
            upstream: upstream.to_string(),
            tarball_src: tarball_src.to_string(),
            owner: owner.to_string(),
            owner_website: owner_website.to_string(),
            maintainer: maintainer,
            build_deps: build_deps,
            runtime_deps: runtime_deps,
            optional_deps: optional_deps,
            conflict_with: conflict_with,
            required_by: required_by,
            provide: provide,
            content: content,
        }
    }
}
