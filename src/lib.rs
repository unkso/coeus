#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate csv;

#[cfg(test)]
#[macro_use] extern crate assert_approx_eq;

/// Parses the fetched sources for identifying lists of players
pub mod parser;
/// Fetches the source that's parsed for identifying lists of players
pub mod fetcher;
/// Contains functionality relating to creating and deleting Cohorts of playhers
pub mod cohort;
/// Contains functionality relating to identifying candidates for the Unknown Soldiers
pub mod identifier;
/// Contains data models
pub mod models;
/// Contains the description of the data persistence layer
pub mod schema;
