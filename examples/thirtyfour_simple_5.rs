// FROM HERE
// https://github.com/stevepryde/thirtyfour/blob/main/thirtyfour/examples/selenium_example.rs

//! Requires selenium running on port 4444:
//!
//!     java -jar selenium-server-standalone-3.141.59.jar
//!
//! Run as follows:
//!
//!     cargo run --example selenium_example

#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};

use std::env::set_var;

use std::time::Duration;
use std::thread;
use std::error::Error;

use thirtyfour::prelude::*;

// use std::error::Error;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    set_var("RUST_LOG", "debug");
    
    color_eyre::install()?;

    let mut _driver = initialize_driver().await?;

    //wait_seconds_of_browser(_driver.clone(), 5).await?;
    thread::sleep(Duration::from_secs(10));


    // let _driver = match _result_driver{

    //     Ok (webdriver) => webdriver,
    //     Err(_e) => return Err(_e),

    // };


    

    // let caps = DesiredCapabilities::chrome();
    // // NOTE: For selenium 3.x, use "http://localhost:4444/wd/hub/session".
    // let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to https://wikipedia.org.
    // _driver.goto("https://wikipedia.org").await?;
    // let elem_form = _driver.find(By::Id("search-form")).await?;

    // // Find element fromlet _driver = initialize_driver().await?;
    // // Type in the search terms.
    // elem_text.send_keys("selenium").await?;
    
    // // Click the search button.
    // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;

    // // Look for header to implicitly wait for the page to load.
    // _driver.find(By::ClassName("firstHeading")).await?;
    // assert_eq!(_driver.title().await?, "Selenium - Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    _driver.quit().await?;

    _driver = initialize_driver().await?;
    thread::sleep(Duration::from_secs(10));
    _driver.quit().await?;


    Ok(())
    
}

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


// cargo run --example  thirtyfour_simple_5