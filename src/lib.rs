#![feature(plugin, custom_derive)]
#![feature(try_trait)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate git2;
extern crate hex;

pub mod webhook;
pub mod config;
pub mod git_repository;
pub mod github;
pub mod errors;
