//use communication::serial_config::PortConfig;
use log::{info, warn};
use misc::device_config::DeviceConfig;
use misc::serial_config::PortConfig;
use protocol::client::HostClient;

pub struct MUClient {
    mu_client: HostClient,
}

/// Режимы стриминга данных от устройства
pub enum StreamingMode {
    /// Без стриминга
    SilentMode = 0,
    /// Отправка данных, в случае изменения их состояния (и наличия)
    OnChangeMode = 1,
    /// Отправка данных периодически, с заданным в настройках (periodicity) периодом
    PeriodicMode = 2,
    /// Отправка данных по требованию
    OnDemandMode = 3,
}

impl MUClient {
    pub fn new(serial_config: &PortConfig) -> Result<Self, String> {
        let client = HostClient::connect(
            serial_config.get_port_name().as_str(),
            serial_config.get_baud_rate(),
            std::time::Duration::from_millis(5000),
        )
        .map_err(|e| e.to_string())?;

        info!("Connection established!");

        Ok(Self { mu_client: client })
    }

    /// Запрос сохраненных в устройстве настроек
    pub fn get_settings_from_device(&mut self, config: &mut DeviceConfig) -> Result<(), String> {
        let group_number = self.mu_client.send_request("get groupnumber")?;

        let music_volume_idx = self.mu_client.send_request("get musicvolume")?;

        let sound_volume_idx = self.mu_client.send_request("get soundvolume")?;

        let load_capacity_idx = self.mu_client.send_request("get loadcapacity")?;

        config.set_group_number(group_number.try_into()?)?;
        config.set_music_volume_idx(music_volume_idx.try_into()?)?;
        config.set_sound_volume_idx(sound_volume_idx.try_into()?)?;
        config.set_load_capacity_idx(load_capacity_idx.try_into()?)?;

        warn!("#Config from device: {}", config);

        Ok(())
    }

    /// Отправка новых настроек на устройство для последующего сохранения
    pub fn push_settings_to_device(&mut self, config: &DeviceConfig) -> Result<(), String> {
        let group_number: String = config.get_group_number().try_into()?;

        // TODO: check response, паттерн "Декоратор"

        self.mu_client
            .send_request(format!("set {}", group_number).as_str())?;

        let music_volume_idx: String = config.get_music_volume_idx().try_into()?;

        self.mu_client
            .send_request(format!("set {}", music_volume_idx).as_str())?;

        let sound_volume_idx: String = config.get_sound_volume_idx().try_into()?;

        self.mu_client
            .send_request(format!("set {}", sound_volume_idx).as_str())?;

        let load_capacity_idx: String = config.get_load_capacity_idx().try_into()?;

        self.mu_client
            .send_request(format!("set {}", load_capacity_idx).as_str())?;

        warn!("#Config to device: {}", config);

        Ok(())
    }

    /// Запрос начала стриминга данных со станции управления
    pub fn start_data_streaming(&mut self, mode: StreamingMode) -> Result<String, String> {
        let response = self
            .mu_client
            .send_request(format!("set mode {}", mode as u8).as_str())?;

        Ok(response)
    }
}
