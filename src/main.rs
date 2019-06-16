extern crate dirs;
extern crate rusqlite;
#[macro_use]
extern crate serenity;
mod bot;
mod dao;

use bot::*;
use dao::init_db;

fn main() {
    init_db();
    let client = init_bot();
    launch_bot(client);
}
