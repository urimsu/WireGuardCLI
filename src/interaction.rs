use crate::check_active;
use crate::connectVPN;
use crate::exitWireGuardCli;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

enum MenuOption {
    CONNECT,
    NEWCONFIG,
    SHOWCONFIG,
    EXIT,
}

fn vpn_status() {
    let is_connected_string = "VPN verbunden: ";
    match check_active::is_vpn_active() {
        Ok(true) => println!("{}{}", is_connected_string, "TRUE".green()),
        Ok(false) => println!("{}{}", is_connected_string, "FALSE".red()),
        Err(e) => println!("ERROR executing wg: {} ", e),
    }
}

fn select_menu() -> MenuOption {
    let options = vec![
        "1. Establish a Connection".to_string(),
        "2. Make a new Connection Config".to_string(),
        "3. Show Config".to_string(),
        "0. Exit".to_string(),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Waehle eine Option")
        .items(&options) // <-- hier 'items', nicht 'item'
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => MenuOption::CONNECT,
        1 => MenuOption::NEWCONFIG,
        2 => MenuOption::SHOWCONFIG,
        3 => MenuOption::EXIT,
        _ => unreachable!(),
    }
}

pub fn interaction() {
    loop {
        println!("\n--------WireGuardCLI--------\n");

        vpn_status();
        let selected_menu = select_menu();
        match selected_menu {
            MenuOption::CONNECT => connectVPN::set_connection(),
            MenuOption::NEWCONFIG => println!("write_new_config()"),
            MenuOption::SHOWCONFIG => println!("show_current_config()"),
            MenuOption::EXIT => {
                exitWireGuardCli::exit_wire_guard_cli();
                break;
            }
        }
    }
}
