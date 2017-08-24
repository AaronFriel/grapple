use github::api;
use git_repository::GitRepository;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Event {
    Ping(api::PingEvent),
    Push(api::PushEvent),
}

impl GitRepository for Event {
    fn repository_name(&self) -> &str {
        match self {
            &Event::Ping(ref e) => e.repository_name(),
            &Event::Push(ref e) => e.repository_name(),
        }
    }

    fn clone_uri(&self) -> &str {
        match self {
            &Event::Ping(ref e) => e.clone_uri(),
            &Event::Push(ref e) => e.clone_uri(),
        }
    }
}