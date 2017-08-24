#[derive(Debug,Deserialize,Clone)]
pub struct Config {
    pub mappings: Vec<RepositoryMapping>,
}

#[derive(Debug,Deserialize,Clone)]
pub struct RepositoryMapping {
    pub from: String,
    pub push_uri: String,
    pub deploy_key: String,
    pub secret: String,
}
