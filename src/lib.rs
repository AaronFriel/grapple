#![feature(custom_attribute)]

extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod github {
  pub mod commit;
  pub mod push_event;
}

pub mod config;

// #[derive(Debug)]
// struct Settings {
//     mapping: Vec<RepositoryMapping>;
// }

#[derive(Debug)]
struct RepositoryMapping {
    from: String,
    to: String,
}


