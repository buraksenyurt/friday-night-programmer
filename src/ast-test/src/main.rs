use ast_test::cli::{Args, Commands};
use ast_test::logger::init_logger;
use ast_test::parser_utility::ParserUtility;
use clap::Parser;
use log::{error, info};
use std::io;
/*
    Test etmek için komut satırından programı aşağıdaki gibi çalıştırabiliriz

    cargo run -- directory ./samples/
    cargo run -- d ./samples/
    cargo run -- dir ./samples/

*/
fn main() -> Result<(), io::Error> {
    init_logger()?;
    info!("Application started");
    let args = Args::parse();

    match args.command {
        Commands::Directory { dir } => {
            if let Err(e) = ParserUtility::generate_from_directory(dir.to_str().unwrap()) {
                error!("Error : {}", e);
            }
        }
    }
    info!("Application stopped");
    Ok(())
}
