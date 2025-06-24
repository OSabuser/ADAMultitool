use crate::config::ConfigIO;
use configparser::ini::Ini;
use std::{fmt::Display, fs};

const MAX_GROUP_NUMBER: u8 = 15;
const MAX_SOUND_VOLUME_IDX: u8 = 4;
const MAX_MUSIC_VOLUME_IDX: u8 = 4;
const MAX_CAPACITY_IDX: u8 = 15;

pub const LOAD_PERSON_VARIANTS: [&str; 16] = [
    "СКРЫТО",
    "240кг 3чел.",
    "320кг 4чел.",
    "400кг 5чел.",
    "450кг 6чел.",
    "525кг 7чел.",
    "630кг 8чел.",
    "800кг 10чел.",
    "800кг 11чел.",
    "1000кг 13чел.",
    "1150кг 15чел.",
    "1275кг 16чел.",
    "1275кг 17чел.",
    "1425кг 18чел.",
    "1600кг 20чел.",
    "1600кг 21чел.",
];

// TODO: from, newtype pattern, tests

#[derive(Debug, Clone)]
pub struct GroupNumber(pub u8);
#[derive(Debug, Clone)]
pub struct MusicVolumeIdx(pub u8);
#[derive(Debug, Clone)]
pub struct SoundVolumeIdx(pub u8);
#[derive(Debug, Clone)]
pub struct LoadCapacityIdx(pub u8);

#[derive(Debug, Clone)]
pub struct DeviceConfig {
    config_name: String,
    group_number: GroupNumber,
    music_volume_idx: MusicVolumeIdx,
    sound_volume_idx: SoundVolumeIdx,
    load_capacity_idx: LoadCapacityIdx,
}

impl DeviceConfig {
    pub fn get_group_number(&self) -> GroupNumber {
        self.group_number.clone()
    }

    pub fn get_music_volume_idx(&self) -> MusicVolumeIdx {
        self.music_volume_idx.clone()
    }

    pub fn get_sound_volume_idx(&self) -> SoundVolumeIdx {
        self.sound_volume_idx.clone()
    }

    pub fn get_load_capacity_idx(&self) -> LoadCapacityIdx {
        self.load_capacity_idx.clone()
    }

    pub fn set_group_number(&mut self, group_number: GroupNumber) -> Result<(), String> {
        if group_number.0 > MAX_GROUP_NUMBER {
            return Err(format!(
                "Group number must be less than {}",
                MAX_GROUP_NUMBER + 1
            ));
        }

        self.group_number = group_number;
        Ok(())
    }

    pub fn set_music_volume_idx(&mut self, music_volume_idx: MusicVolumeIdx) -> Result<(), String> {
        if music_volume_idx.0 > MAX_MUSIC_VOLUME_IDX {
            return Err(format!(
                "Music volume index must be less than {}",
                MAX_MUSIC_VOLUME_IDX + 1
            ));
        }

        self.music_volume_idx = music_volume_idx;
        Ok(())
    }

    pub fn set_sound_volume_idx(&mut self, sound_volume_idx: SoundVolumeIdx) -> Result<(), String> {
        if sound_volume_idx.0 > MAX_SOUND_VOLUME_IDX {
            return Err(format!(
                "Sound volume index must be less than {}",
                MAX_SOUND_VOLUME_IDX + 1
            ));
        }

        self.sound_volume_idx = sound_volume_idx;
        Ok(())
    }

    pub fn set_load_capacity_idx(
        &mut self,
        load_capacity_idx: LoadCapacityIdx,
    ) -> Result<(), String> {
        if load_capacity_idx.0 > MAX_CAPACITY_IDX {
            return Err(format!(
                "Load capacity index must be less than {}",
                MAX_CAPACITY_IDX + 1
            ));
        }

        self.load_capacity_idx = load_capacity_idx;
        Ok(())
    }
}

impl ConfigIO for DeviceConfig {
    fn create_new(name: &str) -> Result<Self, String> {
        let config = Self {
            config_name: name.to_string(),
            group_number: GroupNumber(0),
            music_volume_idx: MusicVolumeIdx(0),
            sound_volume_idx: SoundVolumeIdx(2),
            load_capacity_idx: LoadCapacityIdx(0),
        };
        config.save_parameters()?;
        Ok(config)
    }

    fn create_from_existing(name: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut config = Self {
            config_name: name.to_string(),
            group_number: GroupNumber(0),
            music_volume_idx: MusicVolumeIdx(0),
            sound_volume_idx: SoundVolumeIdx(2),
            load_capacity_idx: LoadCapacityIdx(0),
        };
        config.load_parameters()?;
        Ok(config)
    }

    fn get_config_name(&self) -> String {
        self.config_name.clone()
    }

    fn load_parameters(&mut self) -> Result<(), String> {
        let mut config_instance = Ini::new();
        config_instance.load(format!("configs/device/{}.ini", self.config_name))?;

        match config_instance.getuint("device_settings", "GROUP_NUMBER") {
            Ok(Some(group_number)) => self
                .set_group_number(GroupNumber(group_number as u8))
                .map_err(|e| e.to_string())?,
            _ => return Err("Unable to get group number".to_string()),
        };

        match config_instance.getuint("device_settings", "MUSIC_VOLUME_IDX") {
            Ok(Some(music_volume_idx)) => self
                .set_music_volume_idx(MusicVolumeIdx(music_volume_idx as u8))
                .map_err(|e| e.to_string())?,
            _ => return Err("Unable to get music volume index".to_string()),
        };

        match config_instance.getuint("device_settings", "SOUND_VOLUME_IDX") {
            Ok(Some(sound_volume_idx)) => self
                .set_sound_volume_idx(SoundVolumeIdx(sound_volume_idx as u8))
                .map_err(|e| e.to_string())?,
            _ => return Err("Unable to get sound volume index".to_string()),
        };

        match config_instance.getuint("device_settings", "LOAD_CAPACITY_IDX") {
            Ok(Some(load_capacity_idx)) => self
                .set_load_capacity_idx(LoadCapacityIdx(load_capacity_idx as u8))
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
            Some(self.get_group_number().0.to_string()),
        );
        config_instance.set(
            "device_settings",
            "MUSIC_VOLUME_IDX",
            Some(self.get_music_volume_idx().0.to_string()),
        );
        config_instance.set(
            "device_settings",
            "SOUND_VOLUME_IDX",
            Some(self.get_sound_volume_idx().0.to_string()),
        );
        config_instance.set(
            "device_settings",
            "LOAD_CAPACITY_IDX",
            Some(self.get_load_capacity_idx().0.to_string()),
        );

        return config_instance
            .write(format!("configs/device/{}.ini", self.config_name))
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
            self.group_number.0,
            self.music_volume_idx.0,
            self.sound_volume_idx.0,
            self.load_capacity_idx.0
        )
    }
}

impl TryFrom<String> for GroupNumber {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value.split(':').collect::<Vec<&str>>().to_vec();

        if tokens.len() != 2 {
            return Err("Unable to parse group number".to_string());
        }

        if tokens[0] != "groupnumber" {
            return Err("Unable to parse group number".to_string());
        }

        let group_number = tokens[1].trim().parse::<u8>().map_err(|e| e.to_string())?;
        Ok(GroupNumber(group_number))
    }
}

impl TryInto<String> for GroupNumber {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(format!("groupnumber {}", self.0))
    }
}

impl TryFrom<String> for SoundVolumeIdx {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value.split(':').collect::<Vec<&str>>().to_vec();

        if tokens.len() != 2 {
            return Err("Unable to parse soundvolume number".to_string());
        }

        if tokens[0] != "soundvolume" {
            return Err("Unable to parse soundvolume number".to_string());
        }

        let sound_idx = tokens[1].trim().parse::<u8>().map_err(|e| e.to_string())?;
        Ok(SoundVolumeIdx(sound_idx))
    }
}

impl TryInto<String> for SoundVolumeIdx {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(format!("soundvolume {}", self.0))
    }
}
impl TryFrom<String> for MusicVolumeIdx {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value.split(':').collect::<Vec<&str>>().to_vec();

        if tokens.len() != 2 {
            return Err("Unable to parse musicvolume number".to_string());
        }

        if tokens[0] != "musicvolume" {
            return Err("Unable to parse musicvolume number".to_string());
        }

        let music_idx = tokens[1].trim().parse::<u8>().map_err(|e| e.to_string())?;
        Ok(MusicVolumeIdx(music_idx))
    }
}

impl TryInto<String> for MusicVolumeIdx {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(format!("musicvolume {}", self.0))
    }
}
impl TryFrom<String> for LoadCapacityIdx {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value.split(':').collect::<Vec<&str>>().to_vec();

        if tokens.len() != 2 {
            return Err("Unable to parse loadcapacity number".to_string());
        }

        if tokens[0] != "loadcapacity" {
            return Err("Unable to parse loadcapacity number".to_string());
        }

        let load_idx = tokens[1].trim().parse::<u8>().map_err(|e| e.to_string())?;
        Ok(LoadCapacityIdx(load_idx))
    }
}

impl TryInto<String> for LoadCapacityIdx {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(format!("loadcapacity {}", self.0))
    }
}

impl Display for GroupNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for MusicVolumeIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} %", self.0 * 25)
    }
}

impl Display for SoundVolumeIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} %", self.0 * 25)
    }
}

impl Display for LoadCapacityIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", LOAD_PERSON_VARIANTS[self.0 as usize])
    }
}
