#[derive(Deserialize, Debug)]
pub struct Commit {
    id: String,
    tree_id: String,
    distinct: bool,
    message: String,
    url: String,
    author: CommitAuthor,
    committer: CommitAuthor,
}

#[derive(Deserialize, Debug)]
pub struct CommitAuthor {
    name: String,
    email: String,
    username: String,
}