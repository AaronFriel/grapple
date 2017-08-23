extern crate github_webhook_message_validator;

// use rocket::Outcome;
// use rocket::http::Status;
// use rocket::request::{self, Request, FromRequest};
use std::io::Read;
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

use config::Config;

use git_repository::GitRepository;

pub struct Webhook<T> (pub T);

const LIMIT: u64 = 0x50_000;

impl<T: DeserializeOwned + GitRepository> FromData for Webhook<T>
{
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let config = State::<Config>::from_request(request)
            .into_result()
            .map_err(|_| ())
            .into_outcome(Status::BadRequest)?;

        if !request.content_type().map_or(false, |ct| ct.is_json()) {
            return Outcome::Forward(data);
        }

        let size_limit = request.limits().get("json").unwrap_or(LIMIT);

        let mut payload = Vec::new();

        data.open().take(size_limit).read_to_end(&mut payload)
            .map_err(|_| ())
            .into_outcome(Status::BadRequest)?;

        let json: T = serde_json::from_slice(payload.as_slice())
            .map_err(|_| ())
            .into_outcome(Status::BadRequest)?;

        {
            let repo = json.name();

            let mapping = &config.mappings.iter().find(|m| m.from == repo)
                .ok_or(())
                .into_outcome(Status::BadRequest)?;

            let secret = mapping.secret.as_slice();
            let signature = request.headers().get_one("X-Hub-Signature")
                .ok_or(())
                .into_outcome(Status::BadRequest)?;

            if !validate(secret,&signature.as_bytes(),&payload) {
                return Outcome::Failure((Status::BadRequest, ()))
            }
        }

        Outcome::Success(Webhook(json))
    }
}
