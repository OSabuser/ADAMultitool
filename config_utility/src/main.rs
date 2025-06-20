// UNIX
// $ RUST_LOG=[log_level] ./executable

// WIN64
// $env:RUST_LOG="trace"
// ./executable
pub mod config_client;
pub mod device_config;

use communication::serial_config::PortConfig;
use log::{debug, info};

use config_client::MUClient;
use misc::config::ConfigIO;

use crate::device_config::{GroupNumber, LoadCapacityIdx, MusicVolumeIdx, SoundVolumeIdx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let port_config = PortConfig::create_from_existing("pizero")?;

    let mut device_config = device_config::DeviceConfig::create_from_existing("pi0config")?;

    debug!("#1 Local device config: {}", device_config);
    debug!("#2 Serial port config: {}", port_config);

    let mut client = MUClient::new(&port_config)?;

    client.get_settings_from_device(&mut device_config)?;

    device_config.set_group_number(GroupNumber(0))?;

    device_config.set_music_volume_idx(MusicVolumeIdx(0))?;

    device_config.set_sound_volume_idx(SoundVolumeIdx(2))?;

    device_config.set_load_capacity_idx(LoadCapacityIdx(0))?;

    client.push_settings_to_device(&device_config)?;

    client.get_settings_from_device(&mut device_config)?;

    let response = client.start_data_streaming()?;
    info!("Start data streaming: {}", response);

    Ok(())
}
