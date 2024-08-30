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
        Some(config_dir().unwrap().join("which-key/config.yaml"))
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
                        rows.push(vec![keymap_name, keymap.action.unwrap()]);

                        if let Some(sub_keymaps) = keymap.keymaps {
                            let mut row_keymaps = String::new();
                            let mut row_actions = String::new();

                            for (sub_keymap_name, sub_keymap) in sub_keymaps {
                                row_keymaps.push_str(&format!(" └─ {}\n", sub_keymap_name));
                                row_actions.push_str(&format!("{}\n", sub_keymap.action.unwrap()));
                            }

                            rows.push(vec![row_keymaps, row_actions]);
                        }
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

