use configparser::ini::Ini;
use misc::config::ConfigIO;
use std::{fmt::Display, fs};

const MAX_GROUP_NUMBER: u8 = 15;
const MAX_SOUND_VOLUME_IDX: u8 = 5;
const MAX_MUSIC_VOLUME_IDX: u8 = 5;
const MAX_CAPACITY_IDX: u8 = 16;

#[derive(Debug, Clone)]
pub struct DeviceConfig {
    config_name: String,
    group_number: u8,
    music_volume_idx: u8,
    sound_volume_idx: u8,
    load_capacity_idx: u8,
}

impl DeviceConfig {
    pub fn get_group_number(&self) -> u8 {
        self.group_number
    }

    pub fn get_music_volume_idx(&self) -> u8 {
        self.music_volume_idx
    }

    pub fn get_sound_volume_idx(&self) -> u8 {
        self.sound_volume_idx
    }

    pub fn get_load_capacity_idx(&self) -> u8 {
        self.load_capacity_idx
    }

    pub fn set_group_number(&mut self, group_number: u8) -> Result<(), String> {
        if group_number > MAX_GROUP_NUMBER {
            return Err(format!(
                "Group number must be less than {}",
                MAX_GROUP_NUMBER
            ));
        }

        self.group_number = group_number;
        Ok(())
    }

    pub fn set_music_volume_idx(&mut self, music_volume_idx: u8) -> Result<(), String> {
        if music_volume_idx > MAX_MUSIC_VOLUME_IDX {
            return Err(format!(
                "Music volume index must be less than {}",
                MAX_MUSIC_VOLUME_IDX
            ));
        }

        self.music_volume_idx = music_volume_idx;
        Ok(())
    }

    pub fn set_sound_volume_idx(&mut self, sound_volume_idx: u8) -> Result<(), String> {
        if sound_volume_idx > MAX_SOUND_VOLUME_IDX {
            return Err(format!(
                "Sound volume index must be less than {}",
                MAX_SOUND_VOLUME_IDX
            ));
        }

        self.sound_volume_idx = sound_volume_idx;
        Ok(())
    }

    pub fn set_load_capacity_idx(&mut self, load_capacity_idx: u8) -> Result<(), String> {
        if load_capacity_idx > MAX_CAPACITY_IDX {
            return Err(format!(
                "Load capacity index must be less than {}",
                MAX_CAPACITY_IDX
            ));
        }

        self.load_capacity_idx = load_capacity_idx;
        Ok(())
    }
}

impl ConfigIO for DeviceConfig {
    fn create_default_config() -> Result<Self, String> {
        let config = Self {
            config_name: "default".to_string(),
            group_number: 0,
            music_volume_idx: 0,
            sound_volume_idx: 2,
            load_capacity_idx: 0,
        };
        config.save_parameters()?;
        Ok(config)
    }

    fn create_from_existing_config(name: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut config = Self {
            config_name: name.to_string(),
            group_number: 0,
            music_volume_idx: 0,
            sound_volume_idx: 2,
            load_capacity_idx: 0,
        };
        config.load_parameters()?;
        Ok(config)
    }

    fn get_actual_config_name(&self) -> String {
        self.config_name.clone()
    }

    fn change_config_name(&mut self, name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name should not be empty".into());
        }

        if name
            .chars()
            .all(|arg0: char| char::is_ascii_alphanumeric(&arg0))
        {
            self.config_name = name.to_string();
            return Ok(());
        }

        Err("Name should be alphanumeric".into())
    }
    fn load_parameters(&mut self) -> Result<(), String> {
        let mut config_instance = Ini::new();
        config_instance.load(format!("configs/device/{}.ini", self.config_name))?;

        match config_instance.getuint("device_settings", "GROUP_NUMBER") {
            Ok(Some(group_number)) => self
                .set_group_number(group_number as u8)
                .map_err(|e| e.to_string())?,
            _ => return Err("Unable to get group number".to_string()),
        };

        match config_instance.getuint("device_settings", "MUSIC_VOLUME_IDX") {
            Ok(Some(music_volume_idx)) => self
                .set_music_volume_idx(music_volume_idx as u8)
                .map_err(|e| e.to_string())?,
            _ => return Err("Unable to get music volume index".to_string()),
        };

        match config_instance.getuint("device_settings", "SOUND_VOLUME_IDX") {
            Ok(Some(sound_volume_idx)) => self
                .set_sound_volume_idx(sound_volume_idx as u8)
                .map_err(|e| e.to_string())?,
            _ => return Err("Unable to get sound volume index".to_string()),
        };

        match config_instance.getuint("device_settings", "LOAD_CAPACITY_IDX") {
            Ok(Some(load_capacity_idx)) => self
                .set_load_capacity_idx(load_capacity_idx as u8)
                .map_err(|e| e.to_string())?,
            _ => return Err("Unable to get load capacity index".to_string()),
        };

        Ok(())
    }

    fn save_parameters(&self) -> Result<(), String> {
        let mut config_instance = Ini::new();

        config_instance.set(
            "device_settings",
            "GROUP_NUMBER",
            Some(self.get_group_number().to_string()),
        );
        config_instance.set(
            "device_settings",
            "MUSIC_VOLUME_IDX",
            Some(self.get_music_volume_idx().to_string()),
        );
        config_instance.set(
            "device_settings",
            "SOUND_VOLUME_IDX",
            Some(self.get_sound_volume_idx().to_string()),
        );
        config_instance.set(
            "device_settings",
            "LOAD_CAPACITY_IDX",
            Some(self.get_load_capacity_idx().to_string()),
        );

        return config_instance
            .write(format!("configs/serial/{}.ini", self.config_name))
            .map_err(|e| e.to_string());
    }
    fn list_existing_configs() -> Result<Vec<String>, String> {
        let mut list_of_files = Vec::new();

        if let Ok(entries) = fs::read_dir("configs/device/") {
            for entry in entries {
                if let Ok(dir) = entry {
                    list_of_files.push(
                        dir.path()
                            .display()
                            .to_string()
                            .replace("configs/device/", "")
                            .replace(".ini", ""),
                    );
                } else {
                    break;
                }
            }
            return Ok(list_of_files);
        }

        return Err("Unable to get config file names".to_string());
    }
}

impl Display for DeviceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n Config_name: {}.ini \n Group number: {}, \n Music volume index: {}, \n Sound volume index: {}, \n Load capacity index: {}",
            self.config_name,
            self.group_number,
            self.music_volume_idx,
            self.sound_volume_idx,
            self.load_capacity_idx
        )
    }
}
