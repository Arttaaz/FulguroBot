use serenity::prelude::TypeMapKey;
use std::env;
use serenity::client::{Client, EventHandler};
use serenity::framework::StandardFramework;

//commands use
use crate::bot::commands::*;

#[derive(PartialEq)]
pub enum BetState {
    NotBetting,
    Betting,
    WaitingResult,
}

pub struct BetStateData;
impl TypeMapKey for BetStateData {
    type Value = BetState;
}

struct Handler;

impl EventHandler for Handler {}

pub fn init_bot() -> Client {
    if let Err(unset) = env::var("DISCORD_TOKEN") {
        println!("{}", unset);
        println!("DISCORD_TOKEN variable not set, cannot start FulguroBot.");
        panic!()
    }
    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler).expect("Error creating client");
    client.with_framework(StandardFramework::new()
                            .configure(|c| c.prefix("!"))
                            //add commands here
                            .cmd("noir", noir)
                            .cmd("fulgurobot", fulgurobot));

    {
        let mut data = client.data.lock();
        data.insert::<BetStateData>(BetState::NotBetting);
    }

    client
}

pub fn launch_bot(mut client: Client) {
    if let Err(why) = client.start() {
        println!("Couldn't start FulguroBot: {}", why);
    }
}
