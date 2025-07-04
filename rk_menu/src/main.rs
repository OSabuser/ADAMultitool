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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let args = Args::parse();
    // Чтение сохранённого локально на RPiконфига
    //let mut device_config = DeviceConfig::create_from_existing(args.config_name.as_str())?;

    // Открытие потоков для обслуживания EXT прерываний
    std::thread::spawn(move || {
        let input = Pin::new(S2_BTN as u64);
        input.with_exported(|| {
            input.set_direction(Direction::In)?;
            input.set_edge(Edge::FallingEdge)?;
            let mut poller = input.get_poller()?;
            loop {
                if let Some(pin_value) = poller.poll(1000)? {
                    if pin_value == 0 {
                        println!("S2 pressed!");
                    }
                }
            }
        })
    });

    std::thread::spawn(move || {
        let input = Pin::new(S1_BTN as u64);
        input.with_exported(|| {
            input.set_direction(Direction::In)?;
            input.set_edge(Edge::FallingEdge)?;
            let mut poller = input.get_poller()?;
            loop {
                if let Some(pin_value) = poller.poll(1000)? {
                    if pin_value == 0 {
                        println!("S1 pressed!");
                        //TODO: enigo или передача флага по каналу
                    }
                }
            }
        })
    });

    loop {
        println!("Main thread loop");
        std::thread::sleep(Duration::from_millis(1000));
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
