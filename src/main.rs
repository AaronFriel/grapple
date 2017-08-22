#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate error_chain;

extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

pub mod webhook;
pub mod config;
pub mod has_repository;
pub mod github;
pub mod errors;

use rocket::State;
use rocket_contrib::{Json};

use github::push_event::PushEvent;
use config::Config;

use std::fs::File;
use std::io::prelude::*;
use errors::*;

use webhook::Webhook;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/push", data = "<_pushevent>")]
fn push<'r>(_pushevent: Webhook<Json<PushEvent>>, _config: State<'r, Config>) {
    return;
}

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    rocket::ignite()
        .mount("/", routes![index, push])
        .manage(config)
        .launch();
}

fn load_config() -> Result<Config> {
    let mut errs = Vec::new();

    match load_config_json() {
        Ok(config) => return Ok(config),
        // Err(Error(NoConfiguration, _)) => (),
        Err(e) => errs.push(e),
    }

    match load_config_yaml() {
        Ok(config) => return Ok(config),
        // Err(Error(NoConfiguration, _)) => (),
        Err(e) => errs.push(e),
    }
    Err(Error::from_kind(ErrorKind::ConfigurationError(errs)))
}
static CONFIG_YAML: &'static str = "config.yaml";


fn load_config_yaml() -> Result<Config> {
    let mut file = File::open(CONFIG_YAML).chain_err(|| ErrorKind::ConfigFileError(CONFIG_YAML.to_string()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).chain_err(|| ErrorKind::ConfigFileError(CONFIG_JSON.to_string()))?;

    serde_yaml::from_str(&contents).chain_err(|| ErrorKind::ConfigParseError)
}

static CONFIG_JSON: &'static str = "config.json";

fn load_config_json() -> Result<Config> {
    let mut file = File::open(CONFIG_JSON).chain_err(|| ErrorKind::ConfigFileError(CONFIG_JSON.to_string()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).chain_err(|| ErrorKind::ConfigFileError(CONFIG_JSON.to_string()))?;

    serde_json::from_str(&contents).chain_err(|| ErrorKind::ConfigParseError)
}
