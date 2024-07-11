// #[warn(unused_extern_crates)]
// #[warn(dead_code)]
// use START
// need for log
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};
use std::error::Error;
use std::io::Write;
use std::process;

//
use std::env::set_var;

//need for terminal
use std::io::stdin;
use std::io::stdout;
// double use std::io::Write;

// need for wait_seconds_of_browser
use std::time::Duration;

// need for thread::sleep(Duration::from_secs(waiting_period));
use std::thread;

// need for function name
// use stdext::prelude::*;
// use stdext::function_name;
// need for thirtyfour
#[allow(unused_imports)]
use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

// use FINISHED

// const START

// const WEB_PAGE: &str = "https://www.macrotrends.net";
const WEB_PAGE: &str = "https://wikipedia.org";

// const FINISHED

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    // test for RUST_LOG
    // if env::var("RUST_LOG").is_err() {
    //     env::set_var("RUST_LOG", "debug")
    // }
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
                //function_name!(),
                record.line().unwrap_or(0),
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .init();

    error!("Maybe RUST_LOG NOT enable => RUST_LOG=info < prg >");
    info!("start => env_logger");

    info!("init => tokio::runtime::Builder::new_current_thread");
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    info!("call => fn run ");
    let _ = rt.block_on(run());

    info!("finished => env_logger");
    info!("ciao - We hope to see you soon again");
    process::exit(0);
}

async fn run() -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => fn run ");
    // first init driver
    // let _driver = initialize_driver().await?;
    // let _driver = initialize_driver().await.unwrap();

    // info!("call webpage => {}", WEB_PAGE);
    // _driver.goto(WEB_PAGE).await?;

    //_driver.goto(WEB_PAGE).await.unwrap();
    // goto_web_page(_driver.clone(), WEB_PAGE).await.unwrap();
    // wait_seconds_of_browser(_driver.clone(), 5).await?;
    //
    // wait_seconds_of_browser(_driver.clone(), 5).await.unwrap();

    // use_webdriver_console(_driver.clone()).await.unwrap();
    use_webdriver_console().await.unwrap();

    info!("finished => fn run ");
    Ok(())
}
async fn goto_web_page(
    _driver: WebDriver,
    _web_page: &str,
) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("call webpage => {}", _web_page);
    // _driver.goto(WEB_PAGE).await?;
    _driver.goto(_web_page).await.unwrap();

    Ok(())
}

async fn use_webdriver_console() -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => fn fn use_webdriver_console ");
    // info!("func start => {}", function_name!());
    // info!("func finished => {}", function_name!());
    // wait_seconds_of_browser(_driver.clone(), 10).await?;
    info!("input start => XPath ");

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
        } 

        // start with double point like vim
        else if input.starts_with(':') {
            debug!("enter execute command {}", input);
            // NOT WORK
            // let _ = execute_command(_driver.clone(),&input);

            // FROM HERE - https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust
            if !input.is_empty() {
                input.remove(0); // remove first sign the double point
            }

            // info!("start => execute command => {}", input);
            debug!("start => execute command => {}", input);
            // FROM HERE
            // https://stackoverflow.com/questions/70631899/whats-the-proper-way-to-call-a-async-function-in-another-async-function
            let _ = execute_command(&input).await;
        }
        else{
            println!("{}",input);
        }
    }

    info!("finished => fn fn use_webdriver_console ");
    Ok(())
}

async fn execute_command(_cmd: &String) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => execute_command -> {}", _cmd);

    let mut _driver: Option<Result<WebDriver, WebDriverError>> = None;
    //  wait_seconds_of_browser(_driver.clone(), 10).await?;

    if _cmd == "xpath" {
        debug!("command => {}", _cmd);
    }

    if _cmd == "init" {
        debug!("command => {}", _cmd);

        if let Some(ref _d) = _driver {

            debug!("_driver init => ok");
        }
        else{
            info!("init_driver");
            _driver = Some(initialize_driver().await); 
        }
        
    }

    if _cmd == "open" {
        debug!("command => {}", _cmd);

        let _driver = open_browser().await;
    }

    if _cmd == "close" {
        debug!("command => {}", _cmd);

        // how-i-can-know-if-something-is-initialised-in-rust
        // https://stackoverflow.com/questions/36363693/how-i-can-know-if-something-is-initialised-in-rust

        // if let Some( _d )= _driver {
        //     //println!("x has value: {}", value);

        if let Some(_d) = _driver {
            _d?.quit().await?;
        } else {
            error!("_driver NOT set");
        }
    }
   
    info!("finished => execute_command {}", _cmd);
    Ok(())
}

// FOUND HERE
// https://itehax.com/blog/web-scraping-using-rust
async fn initialize_driver() -> color_eyre::Result<WebDriver, WebDriverError> {
    info!("start => fn initialize_driver ");

    let _caps = DesiredCapabilities::chrome();

    // let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless")?;
    // caps.add_chrome_arg("--no-sandbox")?;
    //  caps.add_chrome_arg("--disable-dev-shm-usage")?;

    let _driver = WebDriver::new("http://localhost:9515", _caps).await?;
    _driver.maximize_window().await?;
    info!("finished => fn initialize_driver ");
    Ok(_driver)
}

async fn wait_seconds_of_browser(
    _driver: WebDriver,
    waiting_period: u64,
) -> color_eyre::Result<(), Box<dyn Error>> {
    // wait for page already load
    info!("start => fn wait_seconds_of_browser ");
    info!("wait n => {} ", waiting_period);
    debug!("Status driver => {:?}", _driver.status().await?);
    debug!("Thread sleep for {} seconds", waiting_period);
    thread::sleep(Duration::from_secs(waiting_period));
    info!("finished => fn wait_seconds_of_browser ");
    Ok(())
}

async fn open_browser() -> color_eyre::Result<WebDriver, WebDriverError> {
    info!("start => fn close_browser ");
    // Always explicitly close the browser.
    // _driver.quit().await?;
    let _driver = initialize_driver().await.unwrap();
    info!("finished => fn close_browser ");

    Ok(_driver)
}

async fn close_browser(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => fn close_browser ");
    // Always explicitly close the browser.
    _driver.quit().await?;
    info!("finished => fn close_browser ");

    Ok(())
}

// RUST_LOG=info cargo run --example webdriver_console_three
// cargo build --example webdriver_console_three

// cargo clippy --fix --allow-dirty --allow-staged examples/webdriver_console_three.rs
// cargo fmt -- --emit=files examples/webdriver_console_three.rs
// cargo check --workspace
