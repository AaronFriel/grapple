use errors::*;

use git2::{Repository, PushOptions, RemoteCallbacks, Cred};
use std::path::{Path};
use std::fs::{self};
use std;

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

pub fn grapple<T: GitRepository>(repo: &T, pull_from_uri: &str, push_to_uri: &str, deploy_key: &str) -> Result<()> {
    let repo = open(repo)?;

    let mut from_remote = repo.remote_anonymous(pull_from_uri)
        .map_err(|e| {println!("{}", e); e})?;

    from_remote.fetch(&[], None, None)
        .map_err(|e| {println!("{}", e); e})?;

    let mut to_remote = repo.remote_anonymous(push_to_uri)
        .map_err(|e| {println!("{}", e); e})?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(
        |_, username, _| Cred::ssh_key(
            username.unwrap_or("grapple"),
            Some(Path::new("/zfsdev/volumes/devhome/.ssh/aelve_rsa.pub")),
            Path::new("/zfsdev/volumes/devhome/.ssh/aelve_rsa"),
            None));

    let mut pushOptions = PushOptions::new();

    pushOptions.remote_callbacks(callbacks);

    to_remote.push(&[], Some(&mut pushOptions))
        .map_err(|e| {println!("{}", e); e})?;

    Ok(())
}