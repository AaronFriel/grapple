extern crate github_webhook_message_validator;

// use rocket::Outcome;
// use rocket::http::Status;
// use rocket::request::{self, Request, FromRequest};
use std::io::Read;

use self::github_webhook_message_validator::validate;

use rocket::{State, Request, Data};
// use rocket::data::DataStream;
use rocket::data::{self, FromData};
use rocket::http::{Status};
use rocket::request::{FromRequest};
use rocket::Outcome::*;

use rocket_contrib::{Json};
// use std;

// use rocket::error::Error;

use config::Config;

pub struct Webhook<T> (pub Json<T>);

// impl<T>

// impl From<

impl<T> FromData for Webhook<T>
    where T: FromData
{
    type Error = ();

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, ()> {
        let config = match State::<Config>::from_request(req) {
            Success(c) => c.inner(),
            _ => return Failure((Status::BadRequest, ()))
        };

        let mut bytes = Vec::new();

        if let Err(_) = data.open().read_to_end(&mut bytes) {
            return Failure((Status::BadRequest, ()))
        };

        let signature = match req.headers().get_one("X-Hub-Signature") {
            Some(hdr) => hdr,
            None => return Failure((Status::BadRequest, ()))
        };

        let secret = b"undefined";

        if !validate(secret,&signature.as_bytes(),&bytes) {
            return Failure((Status::BadRequest, ()))
        }

        println!("Validated webhook");

        T::from_data(unimplemented!(), unimplemented!())
            .map(|t| unimplemented!()).map_failure(|f| unimplemented!())
    }
}
