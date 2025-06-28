// UNIX
// $ RUST_LOG=[log_level] ./executable

// WIN64
// $env:RUST_LOG="trace"
// ./executable
pub mod config_client;

use std::str::FromStr;

use config_client::{MUClient, StreamingMode};
use log::{debug, warn};
use misc::config::ConfigIO;
use misc::serial_config::PortConfig;

use misc::device_config::DeviceConfig;

use clap::Parser;

#[derive(Parser)]
#[command(author = "Akimov Dmitry", name = "config_utility", version = "0.1.0", about, long_about = None)]
struct Args {
    /// Имя конфиг файла
    #[arg(short = 'c', long = "config")]
    config_name: String,
    /// Тип команды: pull - запрос сохраненных в устройстве настроек, push - отправка новых настроек
    #[arg(short = 'm', long = "mode")]
    mode: CommandMode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    warn!("Command mode: {:?}", args.mode);

    env_logger::init();

    let port_config = PortConfig::create_from_existing("pizero")?;

    let mut device_config = DeviceConfig::create_from_existing(args.config_name.as_str())?;

    debug!("#1 Local device config: {}", device_config);
    debug!("#2 Serial port config: {}", port_config);

    let mut client = MUClient::new(&port_config)?;

    match args.mode {
        CommandMode::Pull => pull_command_handler(&mut device_config, &mut client)?,
        CommandMode::Push => push_command_handler(&device_config, &mut client)?,
    }

    Ok(())
}

/// Получение настроек из устройства
fn pull_command_handler(
    user_config: &mut DeviceConfig,
    client: &mut MUClient,
) -> Result<(), String> {
    let mut attempts: u8 = 1;

    // Цикл попыток установить соединение
    'pull_request_loop: loop {
        warn!("Pull request attempt: {}", attempts);
        if client.get_settings_from_device(user_config).is_ok() {
            break 'pull_request_loop;
        }
        attempts += 1;

        if attempts > 3 {
            return Err("Pull request failed!".to_string());
        }
    }
    user_config.save_parameters()?;
    Ok(())
}

/// Отправка настроек на устройство
fn push_command_handler(user_config: &DeviceConfig, client: &mut MUClient) -> Result<(), String> {
    // Цикл попыток установить соединение
    let mut attempts: u8 = 1;
    'push_request_loop: loop {
        warn!("Push request attempt: {}", attempts);
        if client.push_settings_to_device(user_config).is_ok() {
            break 'push_request_loop;
        }
        attempts += 1;

        if attempts > 3 {
            return Err("Push request failed!".to_string());
        }
    }

    attempts = 1;
    'streaming_request_loop: loop {
        warn!("Streaming request attempt: {}", attempts);
        if client
            .start_data_streaming(StreamingMode::OnChangeMode)
            .is_ok()
        {
            break 'streaming_request_loop;
        }
        attempts += 1;

        if attempts > 3 {
            return Err("Streaming request failed!".to_string());
        }
    }

    warn!("Data streaming started!");
    Ok(())
}

#[derive(Clone, Debug)]
enum CommandMode {
    Pull,
    Push,
}

impl FromStr for CommandMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pull" => Ok(CommandMode::Pull),
            "push" => Ok(CommandMode::Push),
            _ => Err(format!("Unknown command mode: {}", s)),
        }
    }
}
