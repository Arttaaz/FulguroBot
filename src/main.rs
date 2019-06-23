#![feature(proc_macro_hygiene)]
extern crate dirs;
extern crate fulgurobot_db;
extern crate serenity;

mod bot;

use bot::*;

fn main() {
    let client = init_bot();
    launch_bot(client);
}
