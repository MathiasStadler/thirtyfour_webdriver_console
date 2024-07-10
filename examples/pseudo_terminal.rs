// FROM HERE
// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};
// use std::error::Error;
use std::io::Write;
// use std::process;

use std::io::stdin;
use std::io::stdout;
// double use std::io::Write;
// use std::process::Command;
fn main() {
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
    info!("start => env_logger");

    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        // shell exit
        if input == "exit" {
            break;
        };

        // start with double point like vim
        if input.starts_with(":") {
            execute_command(&input);
        };

        println!("{}", input);
    }

    fn execute_command(_cmd: &String) {
        info!("start => execute_command");

        let mut _worker = _cmd.clone();

        // FROM HERE - https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust
        if _worker.len() > 0 {
            _worker.remove(0); // remove first
        }

        info!("start => execute command => {}", _worker);

        if _worker == "xpath" {
            debug!("command => {}",_worker);
        }

        info!("finished => execute_command");
    }

    info!("ciao => env_logger");
}

// RUST_LOG=debug cargo run --example --example pseudo_terminal
