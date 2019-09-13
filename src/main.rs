#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;
#[macro_use]
extern crate slog;
extern crate actix_web;
#[macro_use]
extern crate failure;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use actix_web::{http, server, App};
use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod domain;
mod clients;
mod handlers;
mod logging;
mod insults;

use envconfig::Envconfig;

#[derive(Debug)]
pub struct AppState {
    log: slog::Logger,
}

fn main() {
    let log = logging::setup_logging();
    info!(log, "Server Started on localhost:8080");
    server::new(move || {
        App::with_state(AppState {
            log: log.clone(),
        })
        .scope("/rest/v1", |v1_scope| {
            v1_scope.nested("/insults", |activities_scope| {
                activities_scope
                    .resource("", |r| {
                        r.method(http::Method::GET).f(handlers::get_insults);
                        r.method(http::Method::POST)
                            .with_config(handlers::create_insult, |cfg| {
                                (cfg.0).1.error_handler(handlers::json_error_handler);
                            })
                    })
                    .resource("/{activity_id}", |r| {
                        r.method(http::Method::GET).with(handlers::get_insults_by_language);
                    })
            })
        })
        .resource("/health", |r| {
            r.method(http::Method::GET).f(handlers::health)
        })
        .finish()
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run();
}
