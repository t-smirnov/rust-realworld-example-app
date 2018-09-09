#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate config;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod settings;
pub mod http;
pub mod core;
pub mod db;

use settings::Settings;

pub fn init() {
    let settings = Settings::new().unwrap();
    http::api::main(settings);
}