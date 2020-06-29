use curl::easy::Easy;
use std::io::{stdout, Write};
use std::str;

pub fn get(url: &str) -> String {
    let mut result: String = String::from("");
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    result = String::from_utf8(data).unwrap();
    result
}
