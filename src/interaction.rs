use crate::check_active;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

fn vpn_status() {
    let is_connected_string = "VPN verbunden: ";
    match check_active::is_vpn_active() {
        Ok(true) => println!("{}{}", is_connected_string, "TRUE".green()),
        Ok(false) => println!("{}{}", is_connected_string, "FALSE".red()),
        Err(e) => println!("ERROR executing wg: {} ", e),
    }
}

pub fn interaction() {
    let options = vec![
        "1. Establish a Connection".to_string(),
        "2. Make a  new Connection Config".to_string(),
        "3. Show Config".to_string(),
        "0. Exit".to_string(),
    ];

    println!("--------WireGuardCLI--------");

    vpn_status();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Waehle eine Option")
        .items(&options) // <-- hier 'items', nicht 'item'
        .default(0)
        .interact()
        .unwrap();
}
