use super::schema::players::{self, dsl::*};
use super::schema::cohorts;

use diesel::{Connection, SqliteConnection, RunQueryDsl};

#[derive(Debug, Identifiable, Queryable)]
pub struct Player {
    pub id: i32,
    pub cohort_id: i32,
    pub name: String,
    pub rank: i32,
    pub revives: f32,
    pub repairs: f32,
    pub heals: f32,
    pub rounds_played: i32,
    pub squad_score: f32,
    pub flag_captures: i32,
    pub flag_defends: i32,
    pub gamemode_score: f32,
    pub time_played: i32,
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer<'a> {
    pub cohort_id: i32,
    pub name: &'a str,
    pub rank: i32,
    pub revives: f32,
    pub repairs: f32,
    pub heals: f32,
    pub rounds_played: i32,
    pub squad_score: f32,
    pub flag_captures: i32,
    pub flag_defends: i32,
    pub gamemode_score: f32,
    pub time_played: i32,
}

#[derive(Insertable)]
#[table_name = "cohorts"]
pub struct NewCohort<'a> {
    pub name: &'a str,
}

#[derive(Debug, Queryable)]
pub struct Cohort {
    pub id: i32,
    pub name: String,
}

impl Player {
    pub fn all(conn: &SqliteConnection) -> Vec<Self> {
        players.load::<Player>(conn).expect("Could not fetch all players")
    }
}
