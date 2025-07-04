use clap::Parser;

use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use misc::config::ConfigIO;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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

/// SYSFS номер порта кнопки ввода
const S1_BTN: u8 = 65;
/// SYSFS номер порта кнопки выбора
const S2_BTN: u8 = 112;

use std::env;
use std::io::prelude::*;
use std::io::stdout;
use sysfs_gpio::{Direction, Edge, Pin};

fn poll_s2_interrupt(pin: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);
    input.with_exported(|| {
        input.set_direction(Direction::In)?;
        input.set_edge(Edge::FallingEdge)?;
        let mut poller = input.get_poller()?;
        loop {
            if let Some(pin_value) = poller.poll(1000)? {
                if pin_value == 0 {
                    println!("Button pressed!");
                }
            }
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let args = Args::parse();
    // Чтение сохранённого локально на RPiконфига
    //let mut device_config = DeviceConfig::create_from_existing(args.config_name.as_str())?;

    match poll_s2_interrupt(S2_BTN as u64) {
        Ok(()) => println!("Interrupting Complete!"),
        Err(err) => println!("Error: {}", err),
    }

    return Ok(());

    // loop {
    //     match show_main_dialog(&mut device_config) {
    //         Ok(MainMenuStates::ConfigurationState) => {
    //             continue;
    //         }
    //         Ok(MainMenuStates::ExitState) => {
    //             break;
    //         }
    //         Err(e) => return Err(e.into()),
    //     }
    // }
    // device_config.save_parameters()?;
    // Ok(())
}
