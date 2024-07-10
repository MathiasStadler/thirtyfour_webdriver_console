#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};
use std::error::Error;
use std::io::Write;
use std::process;

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    env_logger::builder()
        .format(|buf, record| {
            let warn_style = buf.default_level_style(log::Level::Warn);
            let _timestamp = buf.timestamp();
            writeln!(
                buf,
                // FROM HERE
                // https://docs.rs/env_logger/latest/src/custom_format/custom_format.rs.html#35
                "{}:{}  {warn_style}{}{warn_style:#} - {}",
                // record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .init();

    error!("Maybe RUST_LOG NOT enable => RUST_LOG=info < prg >");
    info!("env_logger: started");

    println!("Hello, world!");

    info!("env_logger: ended");
    process::exit(0);
}
