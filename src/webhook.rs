extern crate github_webhook_message_validator;

use std::convert::From;
use std::io::Read;
use std::ops::Try;

use hex;
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::outcome::{Outcome, IntoOutcome};
use rocket::request::{Request, FromRequest};
use rocket::{State};
use self::github_webhook_message_validator::validate;
use serde::de::DeserializeOwned;
use serde_json;

use config::{Config, RepositoryMapping};
use errors::*;
use git_repository::GitRepository;

pub struct Webhook<T> {
    pub value: T,
    pub mapping: RepositoryMapping,
}

const LIMIT: u64 = 0x50_000;

impl<T: DeserializeOwned + GitRepository> FromData for Webhook<T>
{
    type Error = Error;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let config = State::<Config>::from_request(request)
            .into_result()
            .map_err(|_| Error::from_kind(ErrorKind::ConfigurationNotAvailable))
            .into_outcome(Status::BadRequest)?;

        if !request.content_type().map_or(false, |ct| ct.is_json()) {
            return Outcome::Forward(data);
        }

        let size_limit = request.limits().get("json").unwrap_or(LIMIT);

        let mut payload = Vec::new();

        data.open().take(size_limit).read_to_end(&mut payload)
            .map_err(|e| Error::from_kind(ErrorKind::PayloadReadError(e)))
            .into_outcome(Status::BadRequest)?;

        let json: T = serde_json::from_slice(payload.as_slice())
            .map_err(|e| Error::from_kind(ErrorKind::PayloadParseError(e)))
            .into_outcome(Status::BadRequest)?;

        let mapping = {
            let repo = json.repository_name();

            let mapping = config.mappings.iter().find(|m| m.from == repo)
                .ok_or(Error::from_kind(ErrorKind::MappingLookupError))
                .into_outcome(Status::BadRequest)?;

            let secret = mapping.secret.as_bytes();

            let signature = request.headers().get_one("X-Hub-Signature")
                .ok_or(Error::from_kind(ErrorKind::SignatureHeaderError(None)))
                .into_outcome(Status::BadRequest)?;

            let sig_vec: Vec<u8> = From::from(signature[5..].as_bytes());

            // skip the "sha1="
            let signature_sha: Vec<u8> = hex::FromHex::from_hex(sig_vec)
                .map_err(|e| Error::from_kind(ErrorKind::SignatureHeaderError(Some(e))))
                .into_outcome(Status::BadRequest)?;

            if !validate(secret,&signature_sha,&payload) {
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
