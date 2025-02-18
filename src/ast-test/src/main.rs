use ast_test::cli::{Args, Commands};
use ast_test::parser_utility::ParserUtility;
use clap::Parser;
/*
    Test etmek için komut satırından programı aşağıdaki gibi çalıştırabiliriz

    cargo run -- directory ./samples/
    cargo run -- d ./samples/
    cargo run -- dir ./samples/

*/
fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Directory { dir } => {
            if dir.exists() && dir.is_dir() {
                println!("{:?}", dir);

                for entry in dir.read_dir().expect("Folder read error") {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "cs") {
                            println!("Interface create operation start for {}", path.display());
                            ParserUtility::generate_interface_from_file(path.to_str().unwrap());
                            println!("Interface create operation end for {}", path.display());
                        }
                    }
                }
            } else {
                println!("{:?} does not exist or invalid.", dir);
            }
        }
    }
}
