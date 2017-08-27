use errors::*;

use git2::{Repository, RepositoryInitOptions, Remote, PushOptions, RemoteCallbacks, Cred};
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
                .chain_err(|| Error::from_kind(ErrorKind::RepositoryOpenError))?;
            return attempt_open(repo, attempts-1);
        }
    }

    let mut repo_opts = RepositoryInitOptions::new();
    repo_opts
        .bare(true)
        .no_reinit(true)
        .no_dotgit_dir(false)
        .mkdir(true)
        .mkpath(true);

    // let repo = init_bare(repo.repository_name())

    Ok(Repository::init_opts(repo.repository_name(), &repo_opts)?)
}

const ALL_HEADS: &'static str = "+refs/heads/*:refs/heads/*";
const ALL_TAGS: &'static str = "+refs/tags/*:refs/tags/*";

fn disconnect_head(repo: &Repository) -> Result<()> {
    let head = repo.head()?;

    let oid = head.target().ok_or(Error::from_kind(ErrorKind::RepositoryOpenError))?;

    repo.set_head_detached(oid)?;

    Ok(())
}

const REMOTE_FETCH: &'static str = "fetch";

fn open_remote_fetch<'a>(repo: &'a Repository, uri: &str) -> Result<Remote<'a>> {
    if let Ok(remote) = repo.find_remote(REMOTE_FETCH) {
        Ok(remote)
    } else {
        repo.remote(REMOTE_FETCH, uri)?;

        repo.remote_add_fetch(REMOTE_FETCH, ALL_HEADS)?;
        repo.remote_add_fetch(REMOTE_FETCH, ALL_TAGS)?;

        open_remote_fetch(repo, uri)
    }
}

const REMOTE_PUSH: &'static str = "push";

fn open_remote_push<'a>(repo: &'a Repository, uri: &str) -> Result<Remote<'a>> {
    if let Ok(remote) = repo.find_remote(REMOTE_PUSH) {
        Ok(remote)
    } else {
        repo.remote(REMOTE_PUSH, uri)?;

        // repo.remote_add_push(REMOTE_PUSH, ALL_HEADS)?;
        // repo.remote_add_push(REMOTE_PUSH, ALL_TAGS)?;

        open_remote_push(repo, uri)
    }
}

fn push_glob(repository: &Repository, remote: &mut Remote, glob: &str, push_options: Option<&mut PushOptions>) -> Result<()> {
    let mut refs = repository.references_glob(glob)?;

    let mut push_refs : Vec<&str> = Vec::new();

    for reference in refs.names() {
        if let Ok(name) = reference {
            push_refs.push(name);
        }
    }

    for name in &push_refs {
        println!("{:?}", name);
    }

    Ok(())
}

pub fn grapple<T: GitRepository>(payload: &T, mapping: &RepositoryMapping) -> Result<()> {
    let repo = open(payload)?;

    let mut fetch = open_remote_fetch(&repo, payload.clone_uri())?;

    fetch.fetch(&[], None, None)?;

    let mut push = open_remote_push(&repo, &mapping.push_uri)?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(
        |_, username, _| Cred::ssh_key(
            username.unwrap_or("grapple"),
            Some(Path::new(&mapping.deploy_public_key)),
            Path::new(&mapping.deploy_private_key),
            None));

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    push_glob(&repo, &mut push, "refs/heads/*", Some(&mut push_options))?;
    push_glob(&repo, &mut push, "refs/tags/*", Some(&mut push_options))?;

    Ok(())
}