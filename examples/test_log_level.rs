// FROM HERE
// https://docs.rs/log/latest/log/macro.log_enabled.html

use log::Level::Debug;
use log::{debug, log_enabled};

fn main() {
    if log_enabled!(Debug) {
        debug!("expensive debug data: ");
    }
    if log_enabled!(target: "Global", Debug) {
        debug!(target: "Global", "expensive debug data");
    }

    debug!("expensive debug data: ");
}
