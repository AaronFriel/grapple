#[derive(Debug,Deserialize)]
pub struct Config {
    pub mappings: Vec<RepositoryMapping>,
}

#[derive(Debug,Deserialize)]
pub struct RepositoryMapping {
    pub from: String,
    pub to: String,
    pub secret: Vec<u8>,
}

