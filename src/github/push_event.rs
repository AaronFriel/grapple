use github::commit::Commit;
use github::a_repository::Repository;
// use github::repo::Repository;
// use github::repo::Repository;

#[derive(Deserialize, Debug)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    git_ref: String,

    head: String,

    before: String,

    size: i64,

    distinct_size: i64,

    commits: Vec<Commit>,

    // repository: Repository,
}
