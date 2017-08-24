#[derive(Debug,Deserialize,Clone)]
pub struct Config {
    pub mappings: Vec<RepositoryMapping>,
}

#[derive(Debug,Deserialize,Clone)]
pub struct RepositoryMapping {
    pub from: String,
    pub push_uri: String,
    pub deploy_public_key: String,
    pub deploy_private_key: String,
    pub secret: String,
}
