pub trait GitRepository {
    fn name(&self) -> &str;

    fn clone_uri(&self) -> Option<&str> {
        None
    }

    fn push_uri(&self) -> Option<&str> {
        None
    }
}
