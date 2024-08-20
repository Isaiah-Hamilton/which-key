use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Name of the application to get keymaps from
    #[clap(value_parser)]
    pub name: String,
}


