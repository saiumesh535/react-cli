use std::fs::{create_dir_all, File};
use std::io;
use std::io::{Result as FSResult, Write};
use std::path::Path;
use std::path::PathBuf;

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to read configuration from {}: {}", path.display(), source))]
    ReadConfiguration { source: io::Error, path: PathBuf },
    #[snafu(display("Unable to write result to {}: {}", path.display(), source))]
    WriteResult { source: io::Error, path: PathBuf },
    #[snafu(display("Unable to write result to"))]
    CreateFile { source: io::Error },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn check_if_dir_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn create_dir(path: &str) -> FSResult<()> {
    create_dir_all(path)?;
    Ok(())
}

pub fn write_file_src(dir_name: &String, file_name: &String, content: String) -> Result<()> {
    let dir_path = format!("src/components/{dir_name}", dir_name = dir_name);
    if !check_if_dir_exists(&dir_path) {
        create_dir(&dir_path).context(CreateFile {})?;
    }
    let file_path_str = format!(
        "src/components/{dir_name}/{file_name}",
        dir_name = dir_name,
        file_name = file_name
    );
    let path = Path::new(file_path_str.as_str());
    File::create(path)
        .context(CreateFile {})?
        .write(content.as_bytes())
        .context(CreateFile {})?;
    Ok(())
}
