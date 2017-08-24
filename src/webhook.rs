extern crate github_webhook_message_validator;

// use rocket::Outcome;
// use rocket::http::Status;
// use rocket::request::{self, Request, FromRequest};
use std::io::{self, Read};
use std::ops::Try;

use self::github_webhook_message_validator::validate;

use rocket::outcome::{Outcome, IntoOutcome};
use rocket::request::{Request, FromRequest};
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::{State};
// // use rocket::data::DataStream;
// use rocket::Outcome::{Outcome, IntoOutcome};
// // use rocket::data::{self, Data, FromData};
// use rocket::data::{self, Data, FromData};
// use rocket::http::{Status};
// use rocket::request::Request;
// // use rocket::request::{FromRequest};
// use rocket::response::{self, Responder, content};
// // use rocket::{State, Request, Data};

// use rocket_contrib::{Json};
use serde::de::DeserializeOwned;

use serde_json;

use config::{Config, RepositoryMapping};

use git_repository::GitRepository;

use errors::*;

pub struct Webhook<T> {
    pub value: T,
    pub mapping: RepositoryMapping,
}

const LIMIT: u64 = 0x50_000;

impl<T: DeserializeOwned + GitRepository> FromData for Webhook<T>
{
    type Error = Error;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let mut u = 0;

        let config = State::<Config>::from_request(request)
            .into_result()
            .map_err(|_| Error::from_kind(ErrorKind::ConfigurationNotAvailable))
            .into_outcome(Status::BadRequest)?;


        if !request.content_type().map_or(false, |ct| ct.is_json()) {
            return Outcome::Forward(data);
        }

        let size_limit = request.limits().get("json").unwrap_or(LIMIT);

        let mut payload = Vec::new();

        let len = data.stream_to(&mut payload);


        let json: T = serde_json::from_slice(payload.as_slice())
            .map_err(|e| Error::from_kind(ErrorKind::PayloadParseError(e)))
            .into_outcome(Status::BadRequest)?;


        let mapping = {
            let repo = json.repository_name();


            let mapping = config.mappings.iter().find(|m| m.from == repo)
                .ok_or(Error::from_kind(ErrorKind::MappingLookupError))
                .into_outcome(Status::BadRequest)?;


            let secret = mapping.secret.as_bytes();
            // let signature = request.headers().get_one("X-Hub-Signature")
            //     .ok_or(Error::from_kind(ErrorKind::SignatureHeaderError))
            //     .into_outcome(Status::BadRequest)?;
            let signature = &vec![
                0xcd,
                0x23,
                0x57,
                0x71,
                0x26,
                0xe9,
                0x1e,
                0x53,
                0x14,
                0x3e,
                0xb6,
                0x19,
                0xd9,
                0xa2,
                0x29,
                0x98,
                0x4c,
                0x72,
                0xcb,
                0x59,
            ];


            if !validate(secret,&signature,&payload) {
                println!("Error validating.");
                return Outcome::Failure((Status::BadRequest, Error::from_kind(ErrorKind::ValidationError)))
            }


            mapping
        };

        Outcome::Success(Webhook {
            value: json,
            mapping: mapping.clone(),
        })
    }
}
