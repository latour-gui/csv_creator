extern crate chrono;
extern crate geocoding;
extern crate regex;
extern crate reqwest;
extern crate serde;

// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

pub mod models;
pub mod time;
pub mod forecast;