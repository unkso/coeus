#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;
extern crate regex;

/// Parses the fetched sources for identifying lists of players
mod parser;
/// Fetches the source that's parsed for identifying lists of players
mod fetcher;
/// Contains functionality relating to creating and deleting Cohorts of playhers
mod cohort;
/// Contains functionality relating to identifying candidates for the Unknown Soldiers
mod identifier;
/// Contains the data models that are persisted
mod models;
/// Contains the description of the data persistence layer
mod schema;
