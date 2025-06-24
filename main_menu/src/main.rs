mod main_menu;

use log::{debug, warn};
use main_menu::{MainMenuStates, show_main_dialog};
use misc::config::ConfigIO;
use misc::serial_config::PortConfig;
fn main() -> Result<(), String> {
    env_logger::init();
    let mut config = PortConfig::create_new("default")?;
    loop {
        match show_main_dialog(&mut config) {
            Ok(MainMenuStates::ConfigurationState) => {
                continue;
            }
            Ok(MainMenuStates::ExitState) => {
                debug!("Config state: {}", config);
                break;
            }
            Ok(MainMenuStates::ConnectionRequestState) => {
                debug!("Config state: {}", config);
                warn!("Should launch an utility");
                break;
            }
            Err(e) => return Err(e),
        }
    }

    return Ok(());
}
