use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["keymap", "action"])
        .add_row(vec![
            "cmd-space",
            "opens raycast",
        ])
        .add_row(vec![
            "cmd-q",
            "quits fouced app",
        ]);

    println!("{table}");
}
