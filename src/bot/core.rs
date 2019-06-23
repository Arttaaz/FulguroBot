use std::collections::HashMap;
use serenity::prelude::TypeMapKey;
use std::env;
use serenity::client::{Client, EventHandler};
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;

//commands use
use crate::bot::commands::*;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum BetState {
    NotBetting,
    Betting,
    WaitingResult,
}

pub struct BetStateData;
impl TypeMapKey for BetStateData {
    type Value = HashMap<usize, BetState>;
}

pub struct GameData;
impl TypeMapKey for GameData {
    //(black_player, white player)
    type Value = Vec<Option<(String, String)>>;
}

struct Handler;

impl EventHandler for Handler {}

pub fn init_bot() -> Client {
    if let Err(unset) = env::var("DISCORD_TOKEN") {
        println!("{}", unset);
        println!("DISCORD_TOKEN variable not set, cannot start FulguroBot.");
        panic!()
    }

    group!({
        name: "general",
        options: {},
        commands: [noir, blanc, fulgurobot, coq],
    });

    group!({
        name: "control",
        options: { allowed_roles: ["Animateur", "Team Codeur", "Modération"] },
        commands: [create_game, debut_paris, fin_paris, resultat],
    });

    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler).expect("Error creating client");
    let mut chan_id = serenity::model::id::ChannelId(0);
    {
        let http = &client.cache_and_http.http;
        let guild = http.get_guild(205702304589021184).unwrap();
        for (channel_id, chan) in &guild.channels(&http).unwrap() {
            if chan.name == "testfulgurobot".to_string() {
                chan_id = *channel_id;
            }
        }

    }

    client.with_framework(StandardFramework::new()
                            .configure(|c| c.prefix("!")
                                            .allow_dm(false)
                                            .allowed_channels(vec![chan_id].into_iter().collect()))
                            // .allowed_channels(vec![ChannelId()])
                            //add commands here
                            .group(&GENERAL_GROUP)
                            .group(&CONTROL_GROUP));

    {
        let mut data = client.data.write();
        data.insert::<BetStateData>(HashMap::new());
        data.insert::<GameData>(Vec::new());
    }

    client
}

pub fn launch_bot(mut client: Client) {
    if let Err(why) = client.start() {
        println!("Couldn't start FulguroBot: {}", why);
    }
}
