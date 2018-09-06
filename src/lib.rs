#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

/// Parses the fetched sources for identifying lists of players
mod parser;
/// Fetches the source that's parsed for identifying lists of players
mod fetcher;
/// Contains functionality relating to creating and deleting Cohorts of playhers
mod cohort;
/// Contains functionality relating to identifying candidates for the Unknown Soldiers
mod identifier;
/// Contains data models
mod models;
/// Contains the description of the data persistence layer
mod schema;
