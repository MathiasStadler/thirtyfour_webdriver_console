// #[warn(unused_extern_crates)]
// #[warn(dead_code)]
// use START
// need for log
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};
use std::error::Error;
use std::io::Write;
use std::process;

// need for wait_seconds_of_browser
use std::time::Duration;

// need for thread::sleep(Duration::from_secs(waiting_period));
use std::thread;

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

    info!("init => tokio::runtime::Builder::new_current_thread");
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    info!("call => fn run ");
    let _ = rt.block_on(run());

    info!("finished => env_logger");
    process::exit(0);
}

async fn run() -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => fn run ");
    // first init driver
    // let _driver = initialize_driver().await?;
    let _driver = initialize_driver().await.unwrap();

    info!("call webpage => {}", WEB_PAGE);
    // _driver.goto(WEB_PAGE).await?;
    _driver.goto(WEB_PAGE).await.unwrap();
    // wait_seconds_of_browser(_driver.clone(), 5).await?;
    wait_seconds_of_browser(_driver.clone(), 5).await.unwrap();

    use_webdriver_console(_driver.clone()).await.unwrap();

    info!("finished => fn run ");
    Ok(())
}

async fn use_webdriver_console(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => fn fn use_webdriver_console ");
    wait_seconds_of_browser(_driver.clone(), 10).await?;

    info!("finished => fn fn use_webdriver_console ");
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

    let driver = WebDriver::new("http://localhost:9515", _caps).await?;
    driver.maximize_window().await?;
    info!("finished => fn initialize_driver ");
    Ok(driver)
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

// RUST_LOG=info cargo run --example webdriver_console_two

// cargo clippy --fix --allow-dirty --allow-staged
// cargo fmt -- --emit=files examples/webdriver_console_two.rs
// cargo check --workspace