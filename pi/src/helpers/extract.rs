use flate2::read::GzDecoder;
use std::fs::File;
use std::io::prelude::*;
use tar::Archive;

pub fn extract(path: &str, source_file_name: &str, destination_dir: &str) -> std::io::Result<()> {
    let target = &format!("{}/{}", path, source_file_name);

    let tar_gz = File::open(target)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(destination_dir);
    Ok(())
}
