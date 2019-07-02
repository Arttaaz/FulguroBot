use std::collections::HashMap;
use serenity::prelude::TypeMapKey;
use std::env;
use serenity::client::{Client, EventHandler};
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::utils::MessageBuilder;

//commands use
use crate::bot::commands::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BetState {
    NotBetting,
    Betting,
    WaitingResult,
}
impl From<BetState> for i32 {
    fn from (state: BetState) -> Self {
        match state {
            BetState::NotBetting => 0,
            BetState::Betting => 1,
            BetState::WaitingResult => 2,
        }
    }
}

impl From<i32> for BetState {
    fn from (state: i32) -> Self {
        match state {
            0 => BetState::NotBetting,
            1 => BetState::Betting,
            2 => BetState::WaitingResult,
            _ => BetState::NotBetting,
        }
    }
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
        commands: [noir, blanc, fulgurobot, coq, nb_boost, boost],
    });

    group!({
        name: "control",
        options: { allowed_roles: ["Animateur", "Team Codeur", "Modération"] },
        commands: [create_game, debut_paris, fin_paris, resultat],
    });

    group!({
        name: "debug",
        options: { allowed_roles: ["Team Codeur", "Admin FulguroGo"] },
        commands: [state],
    });

    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler).expect("Error creating client");
    let mut chan_id = serenity::model::id::ChannelId(0);
    {
        let http = &client.cache_and_http.http;
        let guild = http.get_guild(205_702_304_589_021_184).unwrap();
        for (channel_id, chan) in &guild.channels(&http).unwrap() {
            if chan.name == "testfulgurobot" {
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
                            .group(&CONTROL_GROUP)
                            .group(&DEBUG_GROUP));

    {
        let mut data = client.data.write();
        data.insert::<BetStateData>(HashMap::new());
        data.insert::<GameData>(Vec::new());
    }

    restore_context(&client);

    client
}

pub fn launch_bot(mut client: Client) {
    if let Err(why) = client.start() {
        println!("Couldn't start FulguroBot: {}", why);
    }
}

fn restore_context(client: &Client) {
    let conn = fulgurobot_db::connect_db();
    let games = fulgurobot_db::get_games(&conn);
    if !games.is_empty() {
        let mut data = client.data.write();
        let g = data.get_mut::<GameData>().unwrap();
        let mut states : Vec<BetState> = Vec::new();

        for game in games {
            g.push(Some((game.black, game.white)));
            states.push(game.state.into());
        }

        let mut message = MessageBuilder::new();
        message.push("J'ai trouvé ces parties en démarrant :\n");

        for (i, e) in g.iter().enumerate() {
            let e = e.as_ref().unwrap();
            message.push(format!("{} vs {} associé à l'id: ", e.0, e.1));
            message.push_bold_safe(format!("{}\n", i));
        }
        let message = message.build();
        let http = &client.cache_and_http.http;
        let channels = client.cache_and_http.http.get_guild(205_702_304_589_021_184).unwrap().channels(http).unwrap();
        for (_id, chan) in channels {
            if chan.name == "testfulgurobot" {
                let message = chan.say(http, &message);
                if let Err(why) = message {
                    println!("Could not send restore context message: {:?}", why);
                } else if let Ok(m) = message {
                    if let Err(why) = m.channel_id.pin(http, &m) {
                        println!("Could not pin message: {:?}", why);
                    }
                }
                break;
            }
        }

        let size = g.len();
        let b = data.get_mut::<BetStateData>().unwrap();
        for i in 0..size {
            b.insert(i, states[i]);
        }
    }
}
