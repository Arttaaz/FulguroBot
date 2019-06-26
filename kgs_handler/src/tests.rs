use crate::*;

#[test]
fn simple_login() {
    let kgs_client = Client::start(String::from("FulguroBot"), String::from("correcthorsebatterystaple"));
    kgs_client.login(); // should not panic!
}
