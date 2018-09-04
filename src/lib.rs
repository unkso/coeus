#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;

mod parser;
mod scraper;
mod cohort;
mod identifier;
mod models;
mod schema;
