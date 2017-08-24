use github::api::commit::Commit;
use github::api::repository::Repository;

use git_repository::GitRepository;

#[derive(Deserialize, Debug)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    git_ref: String,

    before: String,

    after: String,

    created: bool,

    deleted: bool,

    forced: bool,

    compare: String,

    commits: Vec<Commit>,

    repository: Repository,
}

impl GitRepository for PushEvent {
    fn repository_name(&self) -> &str {
        self.repository.repository_name()
    }

    fn clone_uri(&self) -> &str {
        self.repository.clone_uri()
    }
}

