use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
use std::fs;
use std::io;

pub fn init_logger() -> Result<(), io::Error> {
    let log_file = "app.log";

    if fs::metadata(log_file).is_err() {
        fs::File::create(log_file)?;
    }

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(io::stdout())
        .chain(fern::log_file(log_file)?)
        .apply()
        .expect("Could not set up logger!");

    Ok(())
}
