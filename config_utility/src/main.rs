// Assumes the binary is main:

// $ RUST_LOG=[log_level] ./executable
pub mod device_config;

use device_config::DeviceConfig;
use log::{debug, error, info, trace, warn};
use misc::config::ConfigIO;

fn main() {
    env_logger::init();

    let mut config = DeviceConfig::new("tests");

    config
        .load_parameters()
        .expect("Unable to load parameters!");

    log::error!("\n Loaded parameters: {}", config);
}
