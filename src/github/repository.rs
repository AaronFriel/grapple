use git_repository::GitRepository;

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,
}

impl GitRepository for Repository {
    fn name(&self) -> &str {
        &self.full_name
    }
}
