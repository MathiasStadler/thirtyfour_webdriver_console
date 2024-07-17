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

//
use std::env::set_var;

use std::error::Error;

use std::io::Write;
use std::process;

use std::thread;
use std::time::Duration;

use std::io::stdin;
use std::io::stdout;

use color_eyre::Result;
use std::fmt;

use thirtyfour::ChromiumLikeCapabilities;
use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

// const DEBUG_VEC: bool = false;
// const SHOW_REAL_HTML:bool = false;

// const WEB_PAGE: &str = "https://www.macrotrends.net";
//const STOCK_SYMBOL: &str = "TREX";
const STOCK_SYMBOL: &str = "CROX";

const ACTION_CLICK_INTERACTABLE: &str = "action_click_interactable";
const ACTION_CLICK: &str = "action_click";

// const ACTION_FORM_CLICK_SELECTION_FIELD: &str ="action_form_click_selection_field";
const ACTION_FORM_FILL_FIELD_WITH_SELECT: &str = "action_form_fill_field_with_select";
const ACTION_FORM_FILL_FIELD: &str = "action_form_fill_field";

const ACTION_INTERACTIVE: &str = "action_interactive";
const ACTION_BROWSER_CLOSE: &str = "action_close_browser";

const WEB_XPATH: &[&[&str]] = &[
    //No.,Action,FieldName,xpath
    &[
        "0",
        ACTION_INTERACTIVE,
        STOCK_SYMBOL,
        "/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[1]/span[1]/input",
    ],
    &["1", ACTION_BROWSER_CLOSE, "", ""],
    // &[
    //     "1",
    //     ACTION_FORM_FILL_FIELD_WITH_SELECT,
    //     STOCK_SYMBOL,
    //     "/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[1]/span[1]/input",
    // ],
    // &[
    //     "2",
    //     ACTION_CLICK,
    //     "revenue",
    //     "/html/body/div[1]/div[2]/div[2]/div[2]/div/form/div[1]/div[2]/ul/li[1]/a/span",
    // ],
    // &[
    //     "3",
    //     ACTION_CLICK_INTERACTABLE,
    //     "click",
    //     "/html/body/div[9]/div[1]/div[1]/div/button",
    // ],
    // &[
    //     "4",
    //     ACTION_CLICK,
    //     "click",
    //     "/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a",
    // ],
    // &["5", ACTION_ENTER_FRAME, "enter_frame", "should empty"],
    // &["5",ACTION_CLICK,"click","/html/body/div[3]/div[3]/div[1]/div[1]/ul[1]/li[1]/a"],
];

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
    set_var("RUST_LOG", "debug");
    color_eyre::install()?;

    let mut _call_counter: i32;

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
    let _place: &str = "Place";
    let _driver = initialize_driver().await?;
    let _driver = initialize_driver().await?;

    // _driver.goto(WEB_PAGE).await?;
    thread::sleep(Duration::from_secs(5));

    path_to(_driver.clone()).await?;
    // save_table_to_file_first(_driver.clone()).await?;
    // close_browser(_driver.clone()).await?;

    Ok(())
}

#[allow(dead_code)]
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

async fn action_interactive(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    debug!("wait for page completed load => wait for status from chrome driver");
    debug!("driver=> {:?}", _driver.status().await?);
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

        if input.starts_with(':') {
            debug!("input command modus => {}", input);
            if !input.is_empty() {
                if input.len() > 1 {
                    input.remove(0); // remove first sign, the double point
                    debug!("plain command => {}", input);

                    let _execute_command_result = execute_command(&_driver, &input).await;

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
                    debug!("input no command  => {}", input);
                }
            } else {
                error!("input size is zero");
            }

            debug!("leave command modus => {}", input);
        }
    }
    // old debug!("Thread sleep for {} seconds", waiting_period);
    // old thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}

async fn execute_command(
    driver: &WebDriver,
    cmd: &String,
) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("start => execute_command -> {}", cmd);

    // debug!("execute_command  _cmd => {}", cmd);

    if cmd == "init" {
        debug!("execute_command  _cmd => {}",cmd);

        // let _result_init_driver = init_driver();
    } else if cmd == "close" {
        let result_close_browser = close_browser(driver.clone()).await;
        let _ = match result_close_browser {
            Ok(_web_element) => {
                info!(r#"ACTION_BROWSER_CLOSE => Ok"#);
            }
            Err(_e) => {
                error!(r#"ACTION_BROWSER_CLOSE => Err {_e}"#);
            }
        };

        // let _result_init_driver = init_driver();
    } else if cmd == "open" {
        debug!("execute_command  cmd => {}", cmd);

        let _driver = initialize_driver().await?;

        //let result_driver_goto = driver.goto("https://wikipedia.org").await;
        let result_driver_goto = driver.goto("wikipedia.org").await;
        let _ = match result_driver_goto {
            Ok(_web_element) => {
                info!("ACTION_BROWSER_OPEN => open webpage");
            }
            Err(_e) => {
                error!(r#"ACTION_BROWSER_OPEN => Err {_e}"#);
            }
        };

        // let result_open_browser = open_browser(_driver.clone()).await;
        // let _ = match result_open_browser {
        //     Ok(_web_element) => {
        //         info!(r#"ACTION_BROWSER_OPEN => Ok"#);
        //     }
        //     Err(_e) => {
        //         error!(r#"ACTION_BROWSER_OPEN => Err {_e}"#);
        //     }
        // };
    }
    else{

        info!("Opps!!! Command NOT FOUND {}",cmd);
    }
    debug!("finished => execute_command -> {}", cmd);

    Ok(())
}

async fn path_to(_driver: WebDriver) -> color_eyre::Result<(), Box<dyn Error>> {
    info!("method START => path_to");

    wait_seconds_of_browser(_driver.clone(), 5).await?;

    debug!("XPATH => Browser steps {}", WEB_XPATH.len());

    for field in 0..WEB_XPATH.len() {
        debug!("Next field No.: => {}", WEB_XPATH[field][0]);
        debug!("\tAction => {}", WEB_XPATH[field][1]);
        debug!("\tField => {}", WEB_XPATH[field][2]);

        // if ACTION_BROWSER_CLOSE == WEB_XPATH[field][1] {
        //     debug!(
        //         "Action START =>  ACTION_BROWSER_CLOSE ({})",
        //         WEB_XPATH[field][1]
        //     );
        //     let result_close_browser = close_browser(_driver.clone()).await;
        //     let _ = match result_close_browser {
        //         Ok(_web_element) => {
        //             info!(r#"ACTION_BROWSER_CLOSE => Ok"#);
        //         }
        //         Err(_e) => {
        //             error!(r#"ACTION_BROWSER_CLOSE => Err {_e}"#);

        //             continue;
        //         }
        //     };
        // }

        if ACTION_INTERACTIVE == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_INTERACTIVE ({})",
                WEB_XPATH[field][1]
            );
            action_interactive(_driver.clone()).await?;
        }
        // https://stackoverflow.com/questions/45183797/element-not-interactable-exception-in-selenium-web-automation
        else if ACTION_CLICK_INTERACTABLE == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_CLICK_INTERACTABLE ({})",
                WEB_XPATH[field][1]
            );
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            elem_form.click().await?;
            // wait_seconds_of_browser(_driver.clone(), 10).await?;
            debug!(
                "Action FINISH =>  ACTION_CLICK_INTERACTABLE ({})",
                WEB_XPATH[field][1]
            );
        } else if ACTION_CLICK == WEB_XPATH[field][1] {
            debug!("Action START =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);

            wait_seconds_of_browser(_driver.clone(), 5).await?;

            let elem_form_result: Result<WebElement, WebDriverError> =
                _driver.find(By::XPath(WEB_XPATH[field][3])).await;

            let elem_form = match elem_form_result {
                Ok(_web_element) => {
                    debug!(r#"ACTION_CLICK => web_element found"#);
                    _web_element
                }
                Err(e) => {
                    debug!(r#"Error web_element found"#);
                    eprintln!("Error {}", e);
                    continue;
                }
            };

            //click web element
            elem_form.click().await?;
            debug!("Action FINISH =>  ACTION_CLICK ({})", WEB_XPATH[field][1]);
        } else if ACTION_FORM_FILL_FIELD == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );

            let elem_form: WebElement = _driver.find(By::XPath(WEB_XPATH[field][3])).await?;
            debug!("\tDEBUG => send_keys {}", WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            debug!("\tDEBUG => press enter");
            elem_form.send_keys(Key::Enter).await?;
            wait_seconds_of_browser(_driver.clone(), 5).await?;
            debug!(
                "Action FINISHED =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
        } else if ACTION_FORM_FILL_FIELD_WITH_SELECT == WEB_XPATH[field][1] {
            debug!(
                "Action START =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
            let result_elem_form: Result<WebElement, WebDriverError> =
                _driver.find(By::XPath(WEB_XPATH[field][3])).await;
            let elem_form = match result_elem_form {
                Ok(_web_element) => {
                    debug!(r#"ACTION_FORM_FILL_FIELD_WITH_SELECT=> web_element found"#);
                    _web_element
                }
                Err(e) => {
                    debug!(r#"Error web_element NOT found"#);
                    eprintln!("Error {}", e);
                    continue;
                }
            };
            debug!("\t send_keys {}", WEB_XPATH[field][2]);
            elem_form.send_keys(WEB_XPATH[field][2]).await?;
            debug!("\t select field");
            // elem_form.send_keys(Key::Enter).await?;
            // debug!("\t press enter");

            wait_seconds_of_browser(_driver.clone(), 5).await?;
            debug!(
                "Action FINISH =>  ACTION_FORM_FILL_FIELD ({})",
                WEB_XPATH[field][1]
            );
        } else if ACTION_FORM_FILL_FIELD_WITH_SELECT == WEB_XPATH[field][1] {
            // empty
        }
    }

    wait_seconds_of_browser(_driver.clone(), 5).await?;

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

// FROM HERE
// https://users.rust-lang.org/t/how-to-print-the-type-of-a-variable/101947/2
#[allow(dead_code)]
fn print_type<T>(_: &T) {
    debug!("Type is => {}", std::any::type_name::<T>());
}

/*
rustfmt  ./examples/tokio_finviz_method_five.rs

cargo build --example thirtyfour_interactive_3
cargo run --example thirtyfour_interactive_3
*/
