#[macro_use]
extern crate clap;
extern crate diesel;
extern crate serde_json;
extern crate libcoeus;
extern crate csv;

use clap::App;
use diesel::prelude::*;
use serde_json::Value;

use libcoeus::models::{Cohort, NewPlayer, Player};
use libcoeus::fetcher::{NameFetcher, StatFetcher};
use libcoeus::parser::Parser;

fn main() {
    let cli_config = load_yaml!("cli.yml");
    let _matches = App::from_yaml(cli_config).get_matches();

    let pages = 5;
    let num_per_page = 100;
    let min_rank = 100;
    let max_rank = 120;

    let connection = SqliteConnection::establish("db/local_db.sqlite").unwrap();
    Cohort::new_with_name("the_only", &connection);

    let names = NameFetcher::new()
        .take(pages)
        .fold(Vec::new(), |mut acc, mut val| {
            acc.append(&mut val);
            acc
        });

    let names_len = names.len();
    println!("names len: {}", names_len);

    let stats = StatFetcher::new(names).take(names_len * 50).collect::<Vec<(Value, Value)>>();
    println!("stats len: {}", stats.len());

    NewPlayer::bulk_insert(stats, &connection);

    let players = Player::all(&connection);

    let filtered_players = players
        .iter()
        .filter(|player| player.rank >= min_rank && player.rank <= max_rank)
        .map(|player| (player.name.clone(), player.calculate_teamwork_index(), player.calculate_ptfo_index(), player.calculate_activity_index()))
        .collect::<Vec<(String, f32, f32, f32)>>();

    let file = std::fs::File::create("coeus_list.csv").expect("Could not create file");
    let mut csv_writer = csv::Writer::from_writer(file);

    csv_writer.write_record(&["Name", "Teamwork Index", "PTFO Index", "Activity Index"]).expect("unable to write record");
    for player in filtered_players {
        csv_writer.write_record(&[player.0, format!("{:.2}", player.1), format!("{:.2}", player.2), format!("{:.2}", player.3)]).expect("unable to write record");
    }
    csv_writer.flush().expect("unable to flush writer");
}
