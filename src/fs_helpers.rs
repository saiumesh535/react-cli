use crate::errors::Result;
use crate::errors::*;
use snafu::ResultExt;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn check_if_dir_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn create_dir(path: &str) -> Result<()> {
    create_dir_all(path).context(CreateFile {})?;
    Ok(())
}

pub fn write_file_src(file_name: &String, content: String, dir_path: &String) -> Result<()> {
    if !check_if_dir_exists(dir_path) {
        create_dir(dir_path)?;
    }
    let file_path_str = format!(
        "{dir_path}/{file_name}",
        dir_path = dir_path,
        file_name = file_name
    );
    let path = Path::new(file_path_str.as_str());
    File::create(path)
        .context(CreateFile {})?
        .write_all(content.as_bytes())
        .context(CreateFile {})?;
    Ok(())
}
