mod main_menu;

use communication::serial_config::PortConfig;
use main_menu::{MainMenuStates, show_main_dialog};
use misc::config::ConfigIO;

fn main() -> Result<(), String> {
    let mut port_config = PortConfig::create_default_config()?;

    loop {
        match show_main_dialog(&mut port_config) {
            Ok(MainMenuStates::ConfigurationState) => {
                continue;
            }
            Ok(MainMenuStates::ExitState) => {
                break;
            }
            Ok(MainMenuStates::ConnectionRequestState) => {
                println!("Should launch an utility");
            }
            Err(e) => return Err(e),
        }
    }

    return Ok(());
}
