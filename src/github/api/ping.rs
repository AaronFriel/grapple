use super::repository::Repository;

use git_repository::GitRepository;

#[derive(Deserialize, Debug)]
pub struct PingEvent {
    zen: String,
    hook_id: i64,

    repository: Repository,
}

impl GitRepository for PingEvent {
    fn repository_name(&self) -> &str {
        self.repository.repository_name()
    }

    fn clone_uri(&self) -> &str {
        self.repository.clone_uri()
    }
}
