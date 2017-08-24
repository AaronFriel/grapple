use git_repository::GitRepository;

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,

    pub clone_url: String,
}

impl GitRepository for Repository {
    fn repository_name(&self) -> &str {
        &self.full_name
    }

    fn clone_uri(&self) -> &str {
        self.clone_url.as_str()
    }
}
