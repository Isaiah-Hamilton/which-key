mod cli;

use cli::Args;
use clap::Parser;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

use std::fs::File;
use dirs::config_dir;
use serde::Deserialize;
use serde_yml::from_reader;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Config {
    keymaps: HashMap<String, Keymap>,
}

#[derive(Deserialize, Debug)]
struct Keymap {
    action: Option<String>,
    keymaps: Option<HashMap<String, Keymap>>,
}

fn read_config() -> Result<HashMap<String, Config>, Box<dyn std::error::Error>> {
    let config_path = if cfg!(target_os = "macos") {
        Some(dirs::home_dir().unwrap().join(".config/which-key/config.yaml"))
    } else {
        let mut path = config_dir().unwrap();
        path.push("which-key/config.yaml");
        Some(path)
    };

    if let Some(config_path) = config_path {
        let file = File::open(config_path)?;
        let config: HashMap<String, Config> = from_reader(file)?;
        Ok(config)
    } else {
        Err("No config file found".into())
    }
}

fn main() {
    let args = Args::parse();
    let mut rows: Vec<Vec<String>> = Vec::new();

    match read_config() {
        Ok(config) => {
            for (name, config) in config {
                if name == args.name {
                    for (keymap_name, keymap ) in config.keymaps {
                        rows.push(vec![keymap_name, keymap.action.unwrap()])
                    }
                }
            }
        },
        Err(e) => eprintln!("Error reading config: {}", e),
    }

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["keymap", "action"])
        .add_rows(rows);

    println!("{table}");
}

