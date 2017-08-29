use errors::*;

use std::fs::{self};
use std::path::{Path};

use std::collections::HashMap;

use git2::{Repository, RepositoryInitOptions, Remote, FetchOptions, Direction, FetchPrune, PushOptions, RemoteCallbacks, Cred};

use config::RepositoryMapping;

const ATTEMPTS: u8 = 3;

#[derive(Debug)]
enum RefAction {
    Update,
    Create,
    Delete,
}

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

    Ok(Repository::init_opts(repo.repository_name(), &repo_opts)?)
}

const ALL_HEADS: &'static str = "+refs/heads/*:refs/heads/*";
const ALL_TAGS: &'static str = "+refs/tags/*:refs/tags/*";

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

fn open_remote_push<'a>(repo: &'a Repository, uri: &str) -> Result<Remote<'a>> {
    Ok(repo.remote_anonymous(uri)?)
}

pub fn grapple<T: GitRepository>(payload: &T, mapping: &RepositoryMapping) -> Result<()> {
    let repo = open(payload)?;

    let mut fetch = open_remote_fetch(&repo, payload.clone_uri())?;

    let mut fetch_options = FetchOptions::new();
    fetch_options.prune(FetchPrune::On);

    fetch.fetch(&[], Some(&mut fetch_options), None)?;

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

    fetch.connect(Direction::Fetch)?;

    let fetch_list = fetch.list()?;

    let mut ref_map = HashMap::new();

    println!("Fetch refs:");
    for reference in fetch_list.iter().filter(|r| r.name().starts_with("refs/")) {
        ref_map.insert(reference.name(), reference.oid());
    }


    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(
        |_, username, _| Cred::ssh_key(
            username.unwrap_or("grapple"),
            Some(Path::new(&mapping.deploy_public_key)),
            Path::new(&mapping.deploy_private_key),
            None));

    push.connect_auth(Direction::Push, Some(callbacks), None)?;

    let push_list = push.list()?;

    let mut action_list = Vec::new();

    println!("Push refs:");
    for reference in push_list.iter().filter(|r| r.name().starts_with("refs/")) {
        let ref_name = reference.name();
        let push_oid = reference.oid();
        match ref_map.remove(ref_name) {
            Some(fetch_oid) if fetch_oid != push_oid => action_list.push((ref_name, RefAction::Update)),
            Some(_) => (),
            None => action_list.push((ref_name, RefAction::Delete)),
        }
    }

    for (ref_name, _) in ref_map.iter() {
        action_list.push((ref_name, RefAction::Create));
    }


    let mut fetch = open_remote_fetch(&repo, payload.clone_uri())?;

    let mut push = open_remote_push(&repo, &mapping.push_uri)?;

    for (ref_name, action) in action_list {
        match action {
            RefAction::Create | RefAction::Update => {
                // TODO: collect errors
                let refspec = format!("+{}:{}", ref_name, ref_name);
                fetch.fetch(&[&refspec], Some(&mut fetch_options), None)?;
                push.push(&[&refspec], Some(&mut push_options))?;
            },
            RefAction::Delete => {
                let refspec = format!(":{}", ref_name);
                println!("Delete: {}", ref_name);
                push.push(&[&refspec], Some(&mut push_options))?;
            },
        }
    }

    Ok(())
}