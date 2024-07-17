// FROM HERE
// https://github.com/stevepryde/thirtyfour/blob/main/thirtyfour/examples/selenium_example.rs

//! Requires selenium running on port 9515:
//!
//!
//! Run as follows:
//!
//!      cargo run --example thirtyfour_simple_1

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    
    color_eyre::install()?;

    let caps = DesiredCapabilities::chrome();
    
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("https://wikipedia.org").await?;
    let elem_form = driver.find(By::Id("search-form")).await?;

    // Find element from element.
    let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button.
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    driver.find(By::ClassName("firstHeading")).await?;
    assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await?;

    Ok(())
}


// cargo run --example  thirtyfour_simple_1