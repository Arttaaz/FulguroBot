use crate::*;

#[test]
fn simple_login() {
    let mut kgs_client = KGSClient::new();
    assert!(kgs_client.login(Config::new("login", "pass")).is_err());
    assert!(kgs_client.login(Config::load()).is_ok()); // use valid credentials here
}
