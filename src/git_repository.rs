use errors::*;

use git2::{Repository, PushOptions, RemoteCallbacks, Cred};
use std::path::{Path};
use std::fs::{self};

use config::RepositoryMapping;

const ATTEMPTS: u8 = 3;

pub trait GitRepository {
    fn repository_name(&self) -> &str;

    fn clone_uri(&self) -> &str;
}

fn open<T: GitRepository>(repo: &T) -> Result<Repository> {
    attempt_open(repo, ATTEMPTS)
}

fn attempt_open<T: GitRepository>(repo: &T, attempts: u8) -> Result<Repository> {
    if attempts == 0 {
        return Err(Error::from_kind(ErrorKind::RepositoryOpenError));
    }

    let path = Path::new(repo.repository_name());

    if let Ok(metadata) = path.metadata() {
        if metadata.is_dir() {
            if let Ok(repo) = Repository::open(repo.repository_name()) {
                return Ok(repo)
            } else {
                fs::remove_dir_all(path)
                    .chain_err(|| Error::from_kind(ErrorKind::RepositoryOpenError))?;
                return attempt_open(repo, attempts-1);
            }
        } else {
            fs::remove_file(path)
                .chain_err(|| Error::from_kind(ErrorKind::RepositoryOpenError))?;;
            return attempt_open(repo, attempts-1);
        }
    }

    Ok(Repository::clone(repo.clone_uri(), path)?)
}

pub fn grapple<T: GitRepository>(payload: &T, mapping: &RepositoryMapping) -> Result<()> {
    let repo = open(payload)?;

    let mut from_remote = repo.remote_anonymous(payload.clone_uri())?;

    from_remote.fetch(&[], None, None)?;

    let mut to_remote = repo.remote_anonymous(&mapping.push_uri)?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(
        |_, username, _| Cred::ssh_key(
            username.unwrap_or("grapple"),
            Some(Path::new(&mapping.deploy_public_key)),
            Path::new(&mapping.deploy_private_key),
            None));

    let mut push_options = PushOptions::new();

    push_options.remote_callbacks(callbacks);

    to_remote.push(&[], Some(&mut push_options))?;

    Ok(())
}