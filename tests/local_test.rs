extern crate grapple;


use grapple::config::*;
use grapple::git_repository::*;

pub struct TestValue;

impl GitRepository for TestValue {
    fn repository_name(&self) -> &str {
        "AaronFriel/grapple"
    }

    fn clone_uri(&self) -> &str {
        // "https://github.com/aelve/guide.git"
        "https://github.com/AaronFriel/grapple.git"
    }
}

#[test]
fn test_grapple() {
    let mapping = RepositoryMapping {
        from: "AaronFriel/grapple".to_string(),
        // push_uri: "ssh://git@gitlab.frielforreal.com:58432/aelve/guide.git".to_string(),
        push_uri: "ssh://git@gitlab.frielforreal.com:58432/friel/grapple.git".to_string(),
        deploy_public_key: "/zfsdev/volumes/devhome/.ssh/aelve_rsa.pub".to_string(),
        deploy_private_key: "/zfsdev/volumes/devhome/.ssh/aelve_rsa".to_string(),
        secret: "does not matter".to_string()
    };

    match grapple(&TestValue, &mapping) {
        Err(e) => panic!("{}", e),
        Ok(()) => (),
    }
}