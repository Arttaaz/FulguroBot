extern crate dirs;
extern crate fulgurobot_db;
#[macro_use]
extern crate serenity;

mod bot;

use bot::*;

fn main() {
    let client = init_bot();
    launch_bot(client);
}
