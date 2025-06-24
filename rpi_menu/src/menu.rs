use crossterm::style::Color;
use inquire::validator::Validation;
use inquire::{Confirm, Select, Text};
use misc::device_config::{
    DeviceConfig, GroupNumber, LOAD_PERSON_VARIANTS, LoadCapacityIdx, MusicVolumeIdx,
    SoundVolumeIdx,
};
use terminal_menu::{back_button, button, label, menu, mut_menu, run};
const MAIN_MENU_ITEMS: usize = 5;
const MAIN_MENU_MEMBERS: [&str; MAIN_MENU_ITEMS] = [
    "Номер лифта в группе",
    "Громкость звукового сопровождения",
    "Громкость музыкального сопровождения",
    "Грузоподъемность лифта",
    "Выход с сохранением",
];

pub enum MainMenuStates {
    ConfigurationState,
    ExitState,
}

pub fn show_main_dialog(config: &mut DeviceConfig) -> Result<MainMenuStates, String> {
    // Создание структуры главного меню
    let main_menu = menu(vec![
        label("----------------------").colorize(Color::DarkGreen),
        label("МЕНЮ НАСТРОЕК ИНДИКАТОРА").colorize(Color::DarkGreen),
        label(format!(
            "{} v{}",
            env!("CARGO_CRATE_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .colorize(Color::DarkGreen),
        label(format!("{}", env!("CARGO_PKG_AUTHORS"))).colorize(Color::DarkGreen),
        label("-----------------------").colorize(Color::DarkGreen),
        label("Текущие настройки индикатора").colorize(Color::DarkYellow),
        label(format!("Номер в группе: {}", config.get_group_number())).colorize(Color::DarkYellow),
        label(format!(
            "Громкость звукового сопровождения: {}",
            config.get_sound_volume_idx()
        ))
        .colorize(Color::DarkYellow),
        label(format!(
            "Громкость музыкального сопровождения: {}",
            config.get_music_volume_idx()
        ))
        .colorize(Color::DarkYellow),
        label(format!(
            "Грузоподъемность: {}",
            config.get_load_capacity_idx()
        ))
        .colorize(Color::DarkYellow),
        label("-----------------------").colorize(Color::DarkGreen),
        back_button(MAIN_MENU_MEMBERS[0]),
        back_button(MAIN_MENU_MEMBERS[1]),
        back_button(MAIN_MENU_MEMBERS[2]),
        back_button(MAIN_MENU_MEMBERS[3]),
        button(MAIN_MENU_MEMBERS[4]),
    ]);

    // Отрисовка и навигация по меню
    run(&main_menu);

    // Обработка пользовательского выбора
    match mut_menu(&main_menu).selected_item_name() {
        val if val == MAIN_MENU_MEMBERS[0] => {
            let group_number = show_group_number_dialog()?;
            config.set_group_number(group_number);
            return Ok(MainMenuStates::ConfigurationState);
        }
        val if val == MAIN_MENU_MEMBERS[1] => {
            let vol_idx = show_sound_volume_dialog()?;
            config.set_sound_volume_idx(vol_idx);
            return Ok(MainMenuStates::ConfigurationState);
        }
        val if val == MAIN_MENU_MEMBERS[2] => {
            let vol_idx = show_music_volume_dialog()?;
            config.set_music_volume_idx(vol_idx);

            return Ok(MainMenuStates::ConfigurationState);
        }
        val if val == MAIN_MENU_MEMBERS[3] => {
            let cap_idx = show_capacity_dialog()?;
            config.set_load_capacity_idx(cap_idx);
            return Ok(MainMenuStates::ConfigurationState);
        }
        _ => return Ok(MainMenuStates::ExitState),
    }
}

/// Отображение промпта "Выбор номера группы"
fn show_group_number_dialog() -> Result<GroupNumber, String> {
    let group_names = (0..=15).map(|x| x.to_string()).collect::<Vec<String>>();
    let answer = Select::new("Выбор номера лифта в группе", group_names.clone()).prompt();
    match answer {
        Ok(selection) => {
            let match_index = group_names.iter().position(|x| x == &selection);
            match match_index {
                Some(idx) => Ok(GroupNumber(idx as u8)),
                None => Err("Invalid group number value!".to_string()),
            }
        } //Ok(GroupNumber()), // Ok(selection),
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор громкости звукового сопровождения"
fn show_sound_volume_dialog() -> Result<SoundVolumeIdx, String> {
    let volume_names = (0..=4)
        .map(|x| format!("{}%", 25 * x))
        .collect::<Vec<String>>();
    let answer = Select::new(
        "Выбор громкости звукового сопровождения",
        volume_names.clone(),
    )
    .prompt();
    match answer {
        Ok(selection) => {
            let match_index = volume_names.iter().position(|x| x == &selection);
            match match_index {
                Some(idx) => Ok(SoundVolumeIdx(idx as u8)),
                None => Err("Invalid sound volume value!".to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор громкости музыкального сопровождения"
fn show_music_volume_dialog() -> Result<MusicVolumeIdx, String> {
    let volume_names = (0..=4)
        .map(|x| format!("{}%", 25 * x))
        .collect::<Vec<String>>();
    let answer = Select::new(
        "Выбор громкости музыкального сопровождения",
        volume_names.clone(),
    )
    .prompt();
    match answer {
        Ok(selection) => {
            let match_index = volume_names.iter().position(|x| x == &selection);
            match match_index {
                Some(idx) => Ok(MusicVolumeIdx(idx as u8)),
                None => Err("Invalid music volume value!".to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор грузоподъемности лифта"
fn show_capacity_dialog() -> Result<LoadCapacityIdx, String> {
    let answer = Select::new(
        "Выбор грузоподъемности кабины лифта",
        LOAD_PERSON_VARIANTS.to_vec(),
    )
    .prompt();
    match answer {
        Ok(selection) => {
            let match_index = LOAD_PERSON_VARIANTS.iter().position(|x| x == &selection);
            match match_index {
                Some(idx) => Ok(LoadCapacityIdx(idx as u8)),
                None => Err("Invalid capacity value!".to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}
