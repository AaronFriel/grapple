#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate grapple;

extern crate rocket;
extern crate serde_json;
extern crate serde_yaml;

use std::fs::File;
use std::io::prelude::*;

use grapple::config::Config;
use grapple::errors::*;
use grapple::webhook::Webhook;
use grapple::github;
use grapple::git_repository;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/github", data = "<webhook>", rank = 0)]
fn github<'r>(webhook: Webhook<github::Event>) -> Result<()> {
    match git_repository::grapple(&webhook.value, &webhook.mapping) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
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
        .mount("/", routes![index, github])
        .manage(config)
        .launch();
}

fn load_config() -> Result<Config> {
    let mut errs = Vec::new();

    match load_config_json() {
        Ok(config) => return Ok(config),
        Err(e) => errs.push(e),
    }

    match load_config_yaml() {
        Ok(config) => return Ok(config),
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
