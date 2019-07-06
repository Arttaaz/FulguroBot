use crate::*;

#[test]
fn simple_login() {
    let mut kgs_client = KGSClient::new("theo", "super");
    assert!(kgs_client.login().is_err());
    let mut kgs_client = KGSClient::new("theob", "super");
    assert!(kgs_client.login().is_err());
    let mut kgs_client = KGSClient::new("theob", "super"); // insert valid credentials here
    assert!(kgs_client.login().is_err());
}
