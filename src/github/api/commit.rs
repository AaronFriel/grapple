#[derive(Deserialize, Debug)]
pub struct Commit {
    sha: String,
    message: String,
    author: CommitAuthor,
    url: String,
    distinct: bool,
}

#[derive(Deserialize, Debug)]
pub struct CommitAuthor {
    author: String,
    email: String,
}