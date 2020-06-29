use crate::schemas::store::Store;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Result},
    str,
};

pub fn file_reader(_path: &str) -> Store {
    let file = File::open(_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let store: Store =
        serde_json::from_str(&String::from(contents)).expect("JSON was not well-formatted");

    store
}

pub fn file_writer(data_src: Store, _path: &str) -> Result<()> {
    let mut f = File::create(_path)?;
    f.write_all(serde_json::to_string_pretty(&data_src)?.as_bytes())?;

    f.sync_all()?;
    Ok(())
}
