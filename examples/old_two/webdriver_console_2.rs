// RUN PRG /W full log output
// RUST_LOG=debug cargo run --example thirtyfour_get_margin_data_twenty_four 2>&1 | tee output1.txt

// start cleanup

// RUSTFLAGS=-Wunused-crate-dependencies cargo

// env_logger
// RUST_LOG=info ./main

use color_eyre::eyre::Result;
// thirtyfour 2024
// https://www.zenrows.com/blog/rust-selenium#install-selenium
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};

// use async_recursion::async_recursion;

use std::error::Error;
// use std::fs::File;
use std::io::Write;

#[allow(unused_imports)]
use std::process;

use std::thread;
use std::time::Duration;

use std::env::set_var;

use std::fmt;

use std::io::stdin;
use std::io::stdout;

use thirtyfour::ChromiumLikeCapabilities;
#[allow(unused_imports)]
use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

pub type WebDriverResult<T> = Result<T, WebDriverError>;

#[derive(Debug)]
// #[allow(dead_code)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    // set default log level
    set_var("RUST_LOG", "debug");

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

    error!("RUST_LOG maybe NOT enable");
    error!("Used: => RUST_LOG=info < prg >");

    // for later
    // return Err(Box::new(MyError("Here => Oops".into())));

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let _ = rt.block_on(run());

    info!("env_logger: ended");
    process::exit(0);

    // unreachable expression
    // Ok(())
}

// async fn run() -> color_eyre::Result<(), Box<dyn Error>> {
//     let _driver = initialize_driver().await?;

//     // thread::sleep(Duration::from_secs(10));
//     wait_seconds_of_browser(_driver.clone(), 10).await?;

//     use_webdriver_console(_driver.clone()).await.unwrap();

//     close_browser(_driver.clone()).await?;

//     Ok(())
// }

async fn run() -> color_eyre::Result<(), Box<dyn Error>> {
    let _driver_result: Result<WebDriver, WebDriverError> = initialize_driver().await;

    let _driver = match _driver_result {
        Ok(webdriver) => webdriver,
        Err(_e) => return Err(Box::new(MyError("WebDriverError => {_e}".to_string())).into()),
    };

    // thread::sleep(Duration::from_secs(10));
    wait_seconds_of_browser(_driver.clone(), 5).await?;

    use_webdriver_console(_driver.clone()).await.unwrap();

    close_browser(_driver.clone()).await?;

    Ok(())
}

async fn use_webdriver_console(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => fn use_webdriver_console ");
    wait_seconds_of_browser(_driver.clone(), 5).await?;

    // interactive modus
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        debug!("input => {}", input);

        // shell exit
        if input == "exit" {
            break;
        }
        //end if input == "exit"

        // command start with double point like vim
        else if input.starts_with(':') {
            debug!("found command input => {}", input);
            //extract plain command
            // FROM HERE -
            // https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust
            if !input.is_empty() {
                if input.len() > 1 {
                    input.remove(0); // remove first sign the double point
                    debug!("plain command => {}", input);

                    let _execute_command_result = execute_command(&input).await;

                    let _ = match _execute_command_result {
                        //everything is fine
                        Ok(()) => (),
                        Err(_e) => {
                            return Err(Box::new(MyError(
                                "Error _execute_command => {_e}".to_string(),
                            ))
                            .into())
                        }
                    };
                } else {
                    debug!("plain command only double point  => {}", input);
                }
            }

            debug!("leave command modus => {}", input);
        } // else if input.starts_with(':')
    } // end loop

    info!("finished => fn use_webdriver_console ");
    Ok(())
}

async fn execute_command(_cmd: &String) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => execute_command -> {}", _cmd);

    let _driver: _driver::WebDriver;

    //init webdriver
    if _cmd == "init" {
        debug!("init webdriver {}", _cmd);
        // if check_driver(_driver).await.is_ok() {
        let _result_initialize_driver::<Result<WebDriver, WebDriverError>> = initialize_driver().await;

        let _driver::WebDriver = match _result_initialize_driver {
            Ok(_driver)  => _driver,
            Err(_err) => {
                return Err(Box::new(MyError(
                    "Error result_initialize_driver => {_e}".to_string(),
                ))
                .into())
            }
        };
        //  }// if (check_driver().await) {
    } else if _cmd == "close" {
        debug!("close => browser  {}", _cmd);
        if check_driver(_driver).await.is_ok() {
            let result_close_browser: Result<WebDriver, WebDriverError> =
                close_browser(_driver?).await;
        }
    } else {
        info!("command noz found => {}", _cmd);
    }

    info!("finished => execute_command -> {}", _cmd);
    Ok(())
}

async fn check_driver(_driver: WebDriver) -> color_eyre::Result<WebDriver, WebDriverError> {
    if let Some(_d) = _driver {
        return Ok(_d);
    } else {
        error!("_driver NOT set");
        // return Err(_d)
    }

    // Ok(())
}

// async fn close_browser(_driver: WebDriver) -> color_eyre::Result<WebDriver, WebDriverError> {
//     // Always explicitly close the browser.
//     _driver.quit().await?;

//     Ok(())
// }

// https://github.com/stevepryde/thirtyfour/issues/4?ref=https://githubhelp.com
//
async fn wait_seconds_of_browser(
    _driver: WebDriver,
    waiting_period: u64,
) -> color_eyre::Result<(), Box<dyn Error>> {
    debug!("wait for page completed load => wait for status from chrome driver");
    debug!("driver=> {:?}", _driver.status().await?);
    debug!("Thread sleep for {} seconds", waiting_period);
    thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}

// 25 year
// https://www.macrotrends.net/stocks/charts/TREX/trex/stock-price-history
// sec data
// https://www.sec.gov/cgi-bin/viewer?action=view&cik=1069878&accession_number=0001193125-23-266276&xbrl_type=v#

// FOUND HERE
// https://itehax.com/blog/web-scraping-using-rust

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    info!("initialize_driver - start");

    let mut _caps = DesiredCapabilities::chrome();

    // let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless")?;
    // caps.add_chrome_arg("--no-sandbox")?;
    //  caps.add_chrome_arg("--disable-dev-shm-usage")?;

    _caps.add_arg("--remote-debugging-pipe")?;
    _caps.add_arg("--no-sandbox")?;

    let driver_result = WebDriver::new("http://localhost:9515", _caps).await;

    // let result = WebDriver::new("http://localhost:4444/wd/hub", &caps).await;
    let driver = match driver_result {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    driver.maximize_window().await?;
    info!("initialize_driver - end");
    Ok(driver)
}

async fn close_browser(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => fn close_browser ");
    // Always explicitly close the browser.
    _driver.quit().await?;
    info!("finished => fn close_browser ");

    Ok(())
}

// FROM HERE
// https://users.rust-lang.org/t/how-to-print-the-type-of-a-variable/101947/2
#[allow(dead_code)]
fn print_type<T>(_: &T) {
    debug!("Type is => {}", std::any::type_name::<T>());
}

/*
rustfmt --edition 2024 ./examples/webdriver_console_1.rs

cargo build --example webdriver_console_2

cargo run --example webdriver_console_2
*/
