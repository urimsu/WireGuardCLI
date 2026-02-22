use crate::check_active;
use colored::Colorize;

fn vpn_status() {
    let is_connected_string = "VPN verbunden: ";
    match check_active::is_vpn_active() {
        Ok(true) => println!("{}{}", is_connected_string, "TRUE".green()),
        Ok(false) => println!("{}{}", is_connected_string, "FALSE".red()),
        Err(e) => println!("ERROR executing wg: {} ", e),
    }
}
pub fn interaction() {
    println!("--------WireGuardCLI--------");

    vpn_status();
    println!("1. Establish a Connection");
    println!("2. Make a new Connection Config");
    println!("3. Show Config");
    println!("0. Exit");
}
