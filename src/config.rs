#[derive(Debug,Deserialize)]
pub struct Config {
    pub mappings: Vec<RepositoryMapping>,
}

#[derive(Debug,Deserialize)]
pub struct RepositoryMapping {
    from: String,
    to: String,
    secret: Vec<u8>,
}

