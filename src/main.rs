#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate dirs;
#[macro_use]
extern crate serenity;

mod bot;
mod database;
mod schema;

use bot::*;

fn main() {
    let client = init_bot();
    launch_bot(client);
}
