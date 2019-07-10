extern crate dirs;
extern crate fulgurobot_db;
extern crate serenity;
extern crate kgs_handler;

mod bot;

use bot::*;
// use kgs_handler::*;

fn main() {
    let client = init_bot();
    launch_bot(client);

    // let kgs_client = Client::start(String::from("FulguroBot"), String::from("correcthorsebatterystaple"));
    // kgs_client.login();
}
