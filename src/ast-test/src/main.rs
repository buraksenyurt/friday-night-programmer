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
            if dir.exists() && dir.is_dir() {
                info!("{:?}", dir);

                for entry in dir.read_dir()? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "cs") {
                        info!("Interface create operation start for {}", path.display());
                        if let Err(e) =
                            ParserUtility::generate_interface_from_file(path.to_str().unwrap())
                        {
                            error!("Error {}", e);
                        }
                        info!("Interface create operation end for {}", path.display());
                    }
                }
            } else {
                error!("{:?} does not exist or invalid.", dir);
            }
        }
    }
    info!("Application stopped");
    Ok(())
}
