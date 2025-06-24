use crossterm::style::Color;
use inquire::validator::Validation;
use inquire::{Confirm, Select, Text};
use misc::device_config::DeviceConfig;
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
        label("----------------------").colorize(Color::Green),
        label("МЕНЮ НАСТРОЕК ИНДИКАТОРА").colorize(Color::Green),
        label(format!(
            "{} v{}",
            env!("CARGO_CRATE_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .colorize(Color::Green),
        label(format!("{}", env!("CARGO_PKG_AUTHORS"))).colorize(Color::Green),
        label("-----------------------").colorize(Color::Green),
        label("Текущие настройки индикатора").colorize(Color::Yellow),
        label(format!("Номер в группе: {:?}", config.get_group_number())).colorize(Color::Yellow),
        label(format!(
            "Громкость звукового сопровождения: {:?}",
            config.get_sound_volume_idx()
        ))
        .colorize(Color::Yellow),
        label(format!(
            "Громкость музыкального сопровождения: {:?}",
            config.get_music_volume_idx()
        ))
        .colorize(Color::Yellow),
        label(format!(
            "Грузоподъемность: {:?}",
            config.get_load_capacity_idx()
        ))
        .colorize(Color::Yellow),
        label("-----------------------").colorize(Color::Green),
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
            show_group_number_dialog()?;
            return Ok(MainMenuStates::ConfigurationState);
        }
        val if val == MAIN_MENU_MEMBERS[1] => {
            //*config = show_port_config_dialog()?;
            show_sound_volume_dialog()?;
            return Ok(MainMenuStates::ConfigurationState);
        }
        val if val == MAIN_MENU_MEMBERS[2] => {
            // *config = show_load_config_dialog()?;
            show_music_volume_dialog()?;
            return Ok(MainMenuStates::ConfigurationState);
        }
        val if val == MAIN_MENU_MEMBERS[3] => {
            // *config = show_load_config_dialog()?;
            show_capacity_dialog()?;
            return Ok(MainMenuStates::ConfigurationState);
        }
        _ => return Ok(MainMenuStates::ExitState),
    }
}

/// Отображение промпта "Выбор номера группы"
fn show_group_number_dialog() -> Result<String, String> {
    let group_names = (0..=15).map(|x| x.to_string()).collect::<Vec<String>>();
    let answer = Select::new("Выбор номера лифта в группе", group_names).prompt();
    match answer {
        Ok(selection) => Ok(selection),
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор громкости звукового сопровождения"
fn show_sound_volume_dialog() -> Result<String, String> {
    let volume_names = (0..=4)
        .map(|x| format!("{}%", 25 * x))
        .collect::<Vec<String>>();
    let answer = Select::new("Выбор громкости звукового сопровождения", volume_names).prompt();
    match answer {
        Ok(selection) => Ok(selection),
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор громкости музыкального сопровождения"
fn show_music_volume_dialog() -> Result<String, String> {
    let volume_names = (0..=4)
        .map(|x| format!("{}%", 25 * x))
        .collect::<Vec<String>>();
    let answer = Select::new("Выбор громкости музыкального сопровождения", volume_names).prompt();
    match answer {
        Ok(selection) => Ok(selection),
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор грузоподъемности лифта"
fn show_capacity_dialog() -> Result<String, String> {
    let capacity_names = vec![
        "СКРЫТЬ",
        "320КГ / 4ЧЕЛ",
        "500КГ / 6ЧЕЛ%",
        "720КГ / 10ЧЕЛ%",
        "1100КГ / 16ЧЕЛ%",
    ];
    let answer = Select::new("Выбор грузоподъемности кабины лифта", capacity_names).prompt();
    match answer {
        Ok(selection) => Ok(selection.to_string()),
        Err(e) => return Err(e.to_string()),
    }
}
