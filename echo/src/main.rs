use std::env;

use config::Config;
use cors::CorsHeaders;
use database::{background_task::BackgroundTask, PgDatabase};
use figment::providers::{Format, Json};
use rocket::{catchers, fairing::AdHoc, figment, routes};
use rocket_db_pools::Database;
use routes::{docs, health, pastes, security};
use scanners::InitScanners;

pub mod config;
pub mod cors;
pub mod database;
pub mod models;
pub mod result;
pub mod routes;
pub mod scanners;
pub mod utils;

#[rocket::launch]
fn rocket() -> _ {
    let routes = routes![
        routes::get_root,
        cors::snatcher,
        docs::get_docs,
        docs::get_spec,
        health::health,
        pastes::get_paste,
        pastes::create_paste_simple,
        pastes::create_paste_structured,
        security::get_security,
        security::delete_security,
    ];

    let path = env::var("ECHO_CONFIG").expect("ECHO_CONFIG not set");
    let provider = rocket::Config::figment().merge(Json::file(path));

    rocket::custom(provider)
        .mount("/", routes)
        .attach(AdHoc::config::<Config>())
        .attach(PgDatabase::init())
        .attach(CorsHeaders)
        .attach(InitScanners)
        .attach(BackgroundTask)
        .register("/", catchers![rocket_validation::validation_catcher])
}
