mod main_menu;

use main_menu::{MainMenuStates, show_main_dialog};
use misc::serial_config::PortConfig as serial_config;

fn main() -> Result<(), String> {
    let mut config = serial_config::new();

    loop {
        match show_main_dialog(&mut config) {
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
