#[macro_use] extern crate clap;
extern crate libcoeus;

use clap::App;

fn main() {
    let cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(cli_config).get_matches();
}
