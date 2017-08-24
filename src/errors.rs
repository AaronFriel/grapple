#![allow(unused_doc_comment)]

use std;

use git2;

use rocket::Request;
use rocket::response::{Responder, Response};
use rocket::http::Status;

use serde_json;

error_chain! {
    types{
        Error, ErrorKind, ResultExt, Result;
    }
    links {}
    foreign_links {
    }
    errors {
        ConfigurationError(errs: Vec<Error>) {
            description("configuration file not loaded")
            display("configuration file not loaded, errors:\n{:?}", errs)
        }

        ConfigFileError(f: String) {
            description("failed to load configuration file")
            display("failed to load configuration file '{}'", f)
        }

        ConfigurationNotAvailable {
            description("configuration not available")
        }

        ConfigParseError {
            description("error parsing configuration file")
        }

        PayloadReadError(err: std::io::Error) {
            description("could not read complete payload")
        }

        PayloadParseError(err: serde_json::Error) {
            description("could not parse the webhook payload")
        }

        MappingLookupError {
            description("could not find mapping for webhook received")
        }

        SignatureHeaderError {
            description("could not find signature header for webhook received")
        }

        ValidationError {
            description("webhook received was not valid")
        }

        RepositoryOpenError {
            description("unable to open repository")
        }

        RepositoryGitError(e: git2::Error) {
            description("error running git commands on repository")
        }
    }
}

impl ::std::convert::From<git2::Error> for Error {
    fn from(git_err: git2::Error) -> Error {
        Error::from_kind(ErrorKind::RepositoryGitError(git_err))
        // unimplemented!()
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _req: &Request) -> std::result::Result<Response<'r>, Status> {
        match self {
            Error(_, _) => Err(Status::BadRequest)
        }
    }
}