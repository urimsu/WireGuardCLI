#![allow(non_snake_case)]
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::process::Command;

fn list_configs(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}

fn start_wg(config: &str) {
    let name = config.replace(".conf", "");

    Command::new("wg-quick")
        .arg("up")
        .arg(name)
        .status()
        .expect("wg-quick failed");
}

pub fn set_connection() {
    let configs = list_configs("/etc/wireguard/");

    if configs.is_empty() {
        println!("Keine WireGuard configs gefunden!");
        return;
    }

    println!("{} configs gefunden!", configs.len());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Welche Config soll gestartet werden?")
        .items(&configs)
        .default(0)
        .interact()
        .unwrap();

    let selected = &configs[selection];
    println!("Du hast gewählt: {}", selected);

    start_wg(selected);
}
