use crate::helpers::file::file_reader;
use crate::schemas::store::Store;

pub fn init(db_path: &str) -> Store {
    let mut result = file_reader(db_path);
    result.apps.sort_by_key(|app| app.name.to_string());
    result
}
