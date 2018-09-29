use super::schema::players;
use super::parser::Parser;
use diesel::{prelude::*, result::Error};

use serde_json::Value;
use reqwest::{self};
use super::models::NewPlayer;

pub struct NameFetcher {
    base_uri: &'static str,
    page_pos: u32,
}

impl NameFetcher {
    pub fn new() -> Self {
        NameFetcher {
            base_uri: "https://battlefieldtracker.com/bf1/leaderboards/pc/SiteScore",
            page_pos: 1,
        }
    }
}

impl Iterator for NameFetcher {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let uri = format!("{}?page={}", self.base_uri, self.page_pos);
        self.page_pos += 1;
        let response = reqwest::get(&uri)
            .and_then(|mut result| result.text())
            .expect("unable to fetch response");

        let result = Parser::parse(&response);
        if result.len() > 0 { Some(result) } else { None }
    }
}

pub struct StatFetcher {
    client: reqwest::Client,
    basic_stats_base_uri: &'static str,
    detailed_stats_base_uri: &'static str,
    names: Vec<String>,
    name_index: usize,
}

impl StatFetcher {
    pub fn new(names: Vec<String>) -> Self {
        let mut headers = reqwest::header::Headers::new();
        headers.append_raw("TRN-Api-Key", "ffa481a1-5ba5-40b4-ac99-46bdc0c37d7d");
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        StatFetcher {
            client,
            basic_stats_base_uri:
            "https://battlefieldtracker.com/bf1/api/Stats/BasicStats?platform=3",
            detailed_stats_base_uri:
            "https://battlefieldtracker.com/bf1/api/Stats/DetailedStats?platform=3",
            names,
            name_index: 0,
        }
    }
}

impl Iterator for StatFetcher {
    type Item = (Value, Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.name_index >= self.names.len() {
            return None;
        }

        let basic_stats_uri = format!(
            "{}&displayName={}",
            self.basic_stats_base_uri, self.names[self.name_index]
        );
        let detailed_stats_uri = format!(
            "{}&displayName={}",
            self.detailed_stats_base_uri, self.names[self.name_index]
        );
        self.name_index += 1;

        let basic_stats = self
            .client
            .get(&basic_stats_uri)
            .send()
            .and_then(|mut result| result.json())
            .ok();

        let detailed_stats = self
            .client
            .get(&detailed_stats_uri)
            .send()
            .and_then(|mut result| result.json())
            .ok();

        if basic_stats.is_none() || detailed_stats.is_none() {
            return None;
        }

        ::std::thread::sleep_ms(100);

        Some((basic_stats.unwrap(), detailed_stats.unwrap()))
    }
}

impl<'a> NewPlayer<'a> {
    pub fn bulk_insert(player_stats: Vec<(Value, Value)>, conn: &SqliteConnection) -> Result<usize, Error> {
        let new_players = player_stats.iter().map(|(basic, detailed)| {
            let gamemode_score = if detailed["result"]["gameModeStats"].is_array() {
                detailed["result"]["gameModeStats"]
                    .as_array()
                    .expect("unable to convert game mode stats to array")
                    .iter()
                    .fold(0.0, |acc, v|
                        acc + v["score"].as_f64().expect("could not convert score to f64") as f32,
                    )
            } else {
                0.0
            };

            NewPlayer {
                cohort_id: 1, // TODO: Change this to the proper Cohort ID later
                name: basic["profile"]["displayName"].as_str().unwrap_or("NAME NOT FOUND"),
                rank: basic["result"]["rank"]["number"].as_i64().unwrap_or(0) as i32,
                revives: detailed["result"]["revives"].as_f64().unwrap_or(0.0) as f32,
                repairs: detailed["result"]["repairs"].as_f64().unwrap_or(0.0) as f32,
                heals: detailed["result"]["heals"].as_f64().unwrap_or(0.0) as f32,
                rounds_played: detailed["result"]["roundsPlayed"].as_i64().unwrap_or(0) as i32,
                squad_score: detailed["result"]["squadScore"].as_f64().unwrap_or(0.0) as f32,
                flag_captures: detailed["result"]["flagsCaptured"].as_i64().unwrap_or(0) as i32,
                flag_defends: detailed["result"]["flagsDefended"].as_i64().unwrap_or(0) as i32,
                gamemode_score,
                time_played: basic["result"]["timePlayed"].as_i64().unwrap_or(0) as i32,
            }
        }).collect::<Vec<NewPlayer>>();

        ::diesel::insert_into(players::table).values(new_players).execute(conn)
    }
}

#[cfg(test)]
mod test {
    extern crate diesel_migrations;

    use self::diesel_migrations::run_pending_migrations;

    use diesel::{Connection, SqliteConnection};
    use serde_json::Value;
    use super::{StatFetcher, NameFetcher, NewPlayer};

    fn create_connection() -> SqliteConnection {
        let conn = SqliteConnection::establish(":memory:").expect("Could not create SqliteConnection");

        run_pending_migrations(&conn).expect("Could not run pending migrations");

        conn
    }

    #[test]
    fn it_fetches_pages_of_player_names() {
        let fetcher = NameFetcher::new();
        let page_strings = fetcher
            .take(2)
            .fold(Vec::new(), |mut acc, mut val| {
                acc.append(&mut val);
                acc
            });

        assert_eq!(199, page_strings.len());
        assert_ne!(page_strings[0], page_strings[1]);
    }

    #[test]
    fn it_fetches_json_player_stats() {
        let names = vec!["AuMreaching7King".to_string(), "Silmary_Fr".to_string()];
        let fetcher = StatFetcher::new(names);
        let stats = fetcher.take(2).collect::<Vec<(Value, Value)>>();

        assert_eq!(2, stats.len());
        assert_ne!(stats[0].0, stats[0].1);
        assert_ne!(stats[1].0, stats[1].1);
        assert_ne!(stats[0].0, stats[1].0);
        assert_ne!(stats[0].1, stats[1].1);
    }

    #[test]
    fn it_persists_players_in_the_database_based_on_supplied_cohort() {
        let conn = create_connection();
        let names = vec!["AuMreaching7King".to_string(), "Silmary_Fr".to_string()];
        let players = StatFetcher::new(names).collect::<Vec<(Value, Value)>>();

        let result = NewPlayer::bulk_insert(players, &conn);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2 as usize);
    }
}
