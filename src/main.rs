mod cli;
mod errors;
mod fs_helpers;
mod utils;


use cli::run_cli;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    match run_cli() {
        Ok(_) => {
            println!("Program executed in {:?}", now.elapsed());
        }
        Err(err) => eprintln!("{}", err),
    }
}
