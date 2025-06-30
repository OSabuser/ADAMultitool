use clap::Parser;

use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use misc::config::ConfigIO;
use rppal::gpio::{Event, Gpio, Trigger};
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

/// BCM номер порта кнопки ввода
const IN_BTN: u8 = 7;
/// BCM номер порта кнопки выбора
const SEL_BTN: u8 = 8;

/// Асинхронный обработчик нажатия кнопки ввода
fn in_clicked_handler(event: Event, activity_flag: Arc<Mutex<bool>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::DownArrow, Click).unwrap();
    *activity_flag.lock().unwrap() = true;
}

/// Асинхронный обработчик нажатия кнопки выбора
fn sel_clicked_handler(event: Event, activity_flag: Arc<Mutex<bool>>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Return, Click).unwrap();
    *activity_flag.lock().unwrap() = true;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // Чтение сохранённого локально на RPiконфига
    let mut device_config = DeviceConfig::create_from_existing(args.config_name.as_str())?;

    // Признак активности (пользователь всё ещё устанавливает параметры)
    let is_button_pressed = Arc::new(Mutex::new(false));

    let mut button_in = Gpio::new()?.get(IN_BTN)?.into_input_pullup();
    let mut button_sel = Gpio::new()?.get(SEL_BTN)?.into_input_pullup();

    button_in.set_reset_on_drop(false);
    button_sel.set_reset_on_drop(false);

    let activity_state_hold = is_button_pressed.clone();
    button_in.set_async_interrupt(
        Trigger::FallingEdge,
        Some(Duration::from_millis(50)),
        move |event| in_clicked_handler(event, activity_state_hold.clone()),
    )?;

    let activity_state_hold = is_button_pressed.clone();
    button_sel.set_async_interrupt(
        Trigger::FallingEdge,
        Some(Duration::from_millis(50)),
        move |event| sel_clicked_handler(event, activity_state_hold.clone()),
    )?;

    loop {
        match show_main_dialog(&mut device_config) {
            Ok(MainMenuStates::ConfigurationState) => {
                continue;
            }
            Ok(MainMenuStates::ExitState) => {
                break;
            }
            Err(e) => return Err(e.into()),
        }
    }
    device_config.save_parameters()?;
    Ok(())
}
