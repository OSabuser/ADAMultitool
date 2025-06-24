use clap::Parser;
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use misc::config::ConfigIO;
use rppal::gpio::{Gpio, Level, Trigger};
use std::error::Error;
use std::io;
mod menu;

use menu::{MainMenuStates, show_main_dialog};
use misc::device_config::DeviceConfig;

#[derive(Parser)]
#[command(author = "Akimov Dmitry MU LLC", name = "rpi_menu", version = "0.1.0", about, long_about = None)]
struct Args {
    /// Имя конфиг файла
    #[arg(short = 'c', long = "config")]
    config_name: String,
}

/// BCM номер порта кнопки ввода
pub const IN_BTN: u8 = 7;
/// BCM номер порта кнопки выбора
pub const SEL_BTN: u8 = 8;

/// Обработчик нажатия кнопки ввода
fn in_clicked_handler(lvl: Level) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::DownArrow, Click);
}

/// Обработчик нажатия кнопки выбора
fn sel_clicked_handler(lvl: Level) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Return, Click);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut device_config = DeviceConfig::create_from_existing(args.config_name.as_str())?;

    let mut button_in = Gpio::new()?.get(IN_BTN)?.into_input_pullup();
    let mut button_sel = Gpio::new()?.get(SEL_BTN)?.into_input_pullup();

    button_in.set_reset_on_drop(false);
    button_sel.set_reset_on_drop(false);

    button_in.set_async_interrupt(Trigger::FallingEdge, move |lvl| in_clicked_handler(lvl))?;
    button_sel.set_async_interrupt(Trigger::FallingEdge, move |lvl| sel_clicked_handler(lvl))?;

    loop {
        match show_main_dialog(&mut device_config) {
            Ok(MainMenuStates::ConfigurationState) => {
                continue;
            }
            Ok(MainMenuStates::ExitState) => {
                // TODO: push config to the device
                break;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
