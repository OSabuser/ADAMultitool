use communication::serial_config::PortConfig;
use communication::serial_port::SerialInterface;
use crossterm::style::Color;
use inquire::validator::Validation;
use inquire::{Confirm, Select, Text};
use misc::config::ConfigIO;
use terminal_menu::{back_button, button, label, menu, mut_menu, run};

const MAIN_MENU_ITEMS: usize = 4;
const MAIN_MENU_MEMBERS: [&str; MAIN_MENU_ITEMS] = [
    "Подключиться к устройству",
    "Создать конфигурацию последовательного порта",
    "Загрузить конфигурацию порта из файла",
    "Выход",
];

pub enum MainMenuStates {
    ConnectionRequestState,
    ConfigurationState,
    ExitState,
}

pub fn show_main_dialog(config: &mut PortConfig) -> Result<MainMenuStates, String> {
    // Создание структуры главного меню
    let main_menu = menu(vec![
        label("----------------------").colorize(Color::DarkGreen),
        label(format!(
            "{} v{}",
            env!("CARGO_CRATE_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .colorize(Color::Green),
        label(format!("{}", env!("CARGO_PKG_AUTHORS"))).colorize(Color::Green),
        label("MU LLC, 2025").colorize(Color::DarkGreen),
        label("-----------------------").colorize(Color::Green),
        label("Текущая конфигурация порта").colorize(Color::DarkGreen),
        label(format!("Порт: {}", config.get_port_name())).colorize(Color::DarkGreen),
        label(format!("Скорость: {}", config.get_baud_rate())).colorize(Color::DarkGreen),
        label("-----------------------").colorize(Color::Green),
        back_button(MAIN_MENU_MEMBERS[0]),
        back_button(MAIN_MENU_MEMBERS[1]),
        back_button(MAIN_MENU_MEMBERS[2]),
        button(MAIN_MENU_MEMBERS[3]),
    ]);

    // Отрисовка и навигация по меню
    run(&main_menu);

    // Обработка пользовательского выбора
    match mut_menu(&main_menu).selected_item_name() {
        val if val == MAIN_MENU_MEMBERS[0] => {
            return Ok(MainMenuStates::ConnectionRequestState);
        }
        val if val == MAIN_MENU_MEMBERS[1] => {
            show_port_config_dialog(config)?;
            return Ok(MainMenuStates::ConfigurationState);
        }
        val if val == MAIN_MENU_MEMBERS[2] => {
            show_load_config_dialog(config)?;
            return Ok(MainMenuStates::ConfigurationState);
        }
        _ => return Ok(MainMenuStates::ExitState),
    }
}

/// Отображение диалога создания конфигурации порта
fn show_port_config_dialog(config: &mut PortConfig) -> Result<(), String> {
    let port_selection = show_port_names_dialog()?;
    config.set_port_name(port_selection);

    let baud_selection = show_baudrate_dialog()?;
    config.set_baud_rate(baud_selection);

    if show_save_config_dialog() {
        // set_name -> save_config
        let filename = show_get_filename_dialog()?;
        config.change_config_name(&filename)?;
        config.save_parameters()?;
    }

    return Ok(());
}

fn show_load_config_dialog(config: &mut PortConfig) -> Result<(), String> {
    let config_files = PortConfig::list_existing_configs()?;

    if config_files.len() > 0 {
        let answer = Select::new("Выбор конфигурации", config_files).prompt();
        match answer {
            Ok(selection) => {
                config.change_config_name(&selection.clone())?;
                config.load_parameters()?
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}

/// Отображение промпта "Выбор имени последовательного порта"
fn show_port_names_dialog() -> Result<String, String> {
    let port_names = SerialInterface::get_available_port_names()?;
    let answer = Select::new("Выбор последовательного порта", port_names).prompt();
    match answer {
        Ok(selection) => Ok(selection),
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор скорости порта"
fn show_baudrate_dialog() -> Result<u32, String> {
    let baud_rates = SerialInterface::get_supported_port_speed()?;
    let answer = Select::new("Выбор скорости порта", baud_rates).prompt();
    match answer {
        Ok(selection) => selection
            .parse::<u32>()
            .map_err(|_| "Invalid baud rate!".to_string()),
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Выбор имени конфигурационного файла"
fn show_get_filename_dialog() -> Result<String, String> {
    // Валидатор пользовательского ввода
    let custom_validator = |s: &str| {
        if s.is_empty() {
            return Ok(Validation::Invalid("Empty name".into()));
        }

        if s.chars()
            .all(|arg0: char| char::is_ascii_alphanumeric(&arg0))
        {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(
                "Should use only letters and numbers".into(),
            ))
        }
    };

    let name = Text::new("Имя для нового файла конфигурации:")
        .with_placeholder("should use only ASCII letters and numbers")
        .with_validator(custom_validator)
        .prompt();

    match name {
        Ok(name) => Ok(name),
        Err(e) => return Err(e.to_string()),
    }
}

/// Отображение промпта "Сохранить конфигурацию в файл?"
fn show_save_config_dialog() -> bool {
    let decision = Confirm::new("Сохранить конфигурацию в файл?")
        .with_default(false)
        .prompt();

    match decision {
        Ok(true) => true,
        _ => return false,
    }
}
