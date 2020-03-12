use snafu::Snafu;
use std::env::VarError;
use std::io::Error as IOError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("{}", message))]
    EnvErrorConfig { source: VarError, message: String },
    #[snafu(display("Unable to write result to"))]
    CreateFile { source: IOError },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
