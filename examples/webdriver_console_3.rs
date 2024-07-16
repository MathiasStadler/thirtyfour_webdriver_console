// update here
// RUN PRG /W full log output
// RUST_LOG=debug cargo run --example thirtyfour_get_margin_data_twenty_four 2>&1 | tee output1.txt

// start cleanup

// RUSTFLAGS=-Wunused-crate-dependencies cargo

// env_logger
// RUST_LOG=info ./main

// thirtyfour 2024
// https://www.zenrows.com/blog/rust-selenium#install-selenium
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};

// use async_recursion::async_recursion;

use std::error::Error;
// use std::fs::File;
use std::io::Write;
use std::process;

use std::thread;
use std::time::Duration;

use std::env::set_var;
use std::io::stdin;
use std::io::stdout;
use thirtyfour::ChromiumLikeCapabilities;
#[allow(unused_imports)]
use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

// use thirtyfour::WebDriver;

use color_eyre::Result;
use std::fmt;

const WEB_PAGE: &str = "https://wikipedia.org";

pub type WebDriverResult<T> = Result<T, WebDriverError>;

#[derive(Debug)]
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

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let _ = rt.block_on(run());

    info!("env_logger: ended");
    process::exit(0);
}

async fn run() -> color_eyre::Result<(), Box<dyn Error>> {
    // let _driver = initialize_driver().await?;

    // old can remove
    // _driver.goto(WEB_PAGE).await?;

    // thread::sleep(Duration::from_secs(10));
    //old can remove
    // wait_seconds_of_browser(_driver.clone(), 5).await?;

    let _result_webdriver_console = use_webdriver_console().await;

    let _ = match _result_webdriver_console {
        Ok(()) => () ,
        Err(_err) => {
            return Err(Box::new(MyError(
                "Error result_initialize_driver => {_e}".to_string(),
            ))
            .into())
        }
    };

    // can remove 
    // close_browser(_driver.clone()).await?;

    Ok(())
}

async fn use_webdriver_console() -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => use_webdriver_console ");
    
    // can remove 
    // wait_seconds_of_browser(_driver.clone(), 5).await?;

    // interactive modus
    // loop {
       
    // init console
    let result_init_driver = init_driver().await;
    let _driver::<WebDriver,WebDriverError> = match result_init_driver {

        Ok(_driver::WebDriver) => Ok(_driver),
        Err(_e::WebDriverError) => Err(_e),
        
    };

       
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        debug!("input => {}", input);

        // shell exit
        //if input == "exit" {
          //  break;
        //}
        // command start with double point like vim
        if input.starts_with(':') {
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
                    debug!("only double point without command  => {}", input);
                }
            }

            debug!("leave command modus => {}", input);
        } // else if input.starts_with(':')


    // } // end of interactive loop
      //end if input == "exit"

    info!("finished => use_webdriver_console ");
    Ok(())
}

async fn execute_command(_cmd: &String) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => execute_command -> {}", _cmd);

    debug!("execute_command  _cmd => {}", _cmd);

    if _cmd == "init" {
        debug!("execute_command  _cmd => {}", _cmd);

        let _result_init_driver = init_driver();
    }
    debug!("finished => execute_command -> {}", _cmd);

    Ok(())
}

async fn close_browser(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    // Always explicitly close the browser.
    _driver.quit().await?;

    Ok(())
}

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

async fn init_driver() -> Result<WebDriver, WebDriverError> {
    info!("initialize_driver - start");

    let mut _caps = DesiredCapabilities::chrome();

    // let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless")?;
    // caps.add_chrome_arg("--no-sandbox")?;
    // caps.add_chrome_arg("--disable-dev-shm-usage")?;

    _caps.add_arg("--remote-debugging-pipe")?;
    _caps.add_arg("--no-sandbox")?;

    let result_driver::<Result<WebDriver::WebDriverError>> =
        WebDriver::new("http://localhost:9515", _caps).await;

    // let result = WebDriver::new("http://localhost:4444/wd/hub", &caps).await;
    let driver::WebDriver = match result_driver {
        Ok(_driver::<WebDriver>) => _driver,
        Err(_error::<WebDriverError>) => return Err(_error),
    };

    driver.maximize_window().await?;
    info!("initialize_driver - end");
    Ok(driver)
}

// FROM HERE
// https://users.rust-lang.org/t/how-to-print-the-type-of-a-variable/101947/2
#[allow(dead_code)]
fn print_type<T>(_: &T) {
    debug!("Type is => {}", std::any::type_name::<T>());
}

/*
rustfmt --edition 2024 ./examples/webdriver_console_1.rs

cargo build --example webdriver_console_1

cargo run --example webdriver_console_1
*/