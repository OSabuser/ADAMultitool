// UNIX
// $ RUST_LOG=[log_level] ./executable

// WIN64
// $env:RUST_LOG="trace"
// ./executable
pub mod device_config;

use communication::serial_config::PortConfig;
use log::{debug, error, info, trace, warn};
use misc::config::ConfigIO;
use protocol::host::HostClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let port_config = PortConfig::create_from_existing_config("pizero")?;

    debug!(
        "Loaded port config parameters are: {}, {}",
        port_config.get_port_name(),
        port_config.get_baud_rate()
    );

    let mut client = HostClient::connect(
        port_config.get_port_name().as_str(),
        port_config.get_baud_rate(),
        std::time::Duration::from_millis(5000),
    )
    .unwrap();

    info!("Connected to device!");

    let response = client.send_request("set mode 2").unwrap();

    info!("Received response: {}", response);

    Ok(())
}
