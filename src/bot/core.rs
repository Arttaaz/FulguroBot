use crate::bot::consts::DISCORD_EMBED_COLOR;
use std::{ collections::HashMap, env };
use serenity::{
    client::{Client, EventHandler},
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    model::channel::Message,
    utils::MessageBuilder,
    prelude::TypeMapKey,
};

use crate::bot::consts::DISCORD_CHANNEL_ID;
//commands use
use crate::bot::commands::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    type Value = Vec<Option<(String, String, Option<Message>)>>;
}

struct Handler;

impl EventHandler for Handler {}

group!({
    name: "general",
    options: {},
    commands: [noir, black, blanc, white, fulgurobot, help, coq, shell, nb_recharge, nb_refill, recharge, refill, etat, state, /*give*/]
});

group!({
    name: "control",
    options: { allowed_roles: ["Animateur", "Team Codeur", "Modération", "Admin FulguroGo"] },
    commands: [create_game, debut_paris, fin_paris, resultat],
});

group!({
    name: "debug",
    options: { allowed_roles: ["Team Codeur", "Admin FulguroGo"] },
    commands: [],
});

group!({
    name: "hisokah",
    options: { allowed_roles: ["Admin FulguroGo"] },
    commands: [boost],
});


pub fn init_bot() -> Client {
    if let Err(unset) = env::var("DISCORD_TOKEN") {
        println!("{}", unset);
        println!("DISCORD_TOKEN variable not set, cannot start FulguroBot.");
        panic!()
    }



    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap(), Handler).expect("Error creating client");
    let chan_id = serenity::model::id::ChannelId(DISCORD_CHANNEL_ID);

    client.with_framework(StandardFramework::new()
                            .bucket("basic", |b| b.delay(2).time_span(10).limit(3))
                            .configure(|c| c.prefix("!")
                                            .allow_dm(false)
                                            .allowed_channels(vec![chan_id].into_iter().collect()))
                            //add commands here
                            .group(&GENERAL_GROUP)
                            .group(&CONTROL_GROUP)
                            .group(&DEBUG_GROUP)
                            .group(&HISOKAH_GROUP));

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

    // if games were found in the database, we need to restore the context
    if !games.is_empty() {
        let mut data = client.data.write();
        let g = data.get_mut::<GameData>().unwrap();
        let mut states : Vec<BetState> = Vec::new();

        // fill GameData array
        for game in games {
            g.push(Some((game.black, game.white, None)));
            states.push(game.state.into());
        }

        // send feedback
        let http = &client.cache_and_http.http;
        let chan = serenity::model::id::ChannelId(DISCORD_CHANNEL_ID);
        let message = chan.send_message(http, |m| {
            m.embed(|e| {
                e.title("J'ai trouvé ces parties en démarrant :")
                 .color(DISCORD_EMBED_COLOR);

                 let mut text = MessageBuilder::new();
                 for (i, e) in g.iter().enumerate() {
                     let e = e.as_ref().unwrap();
                     text.push(format!("{} vs {} associé à l'id: ", e.0, e.1));
                     text.push_bold_safe(format!("{}\n", i));
                 }
                 let text = text.build();
                 e.description(text);

                e
            });
            m
        });
        if let Err(why) = message {
            println!("Could not send restore context message: {:?}", why);
        } else if let Ok(m) = message {
            // unpin all previous pins and pin last message
            for pin in m.channel_id.pins(http).unwrap() {
                if let Err(why) = m.channel_id.unpin(http, &pin) {
                    println!("Could not unpin message: {:?}", why);
                }
            }
            if let Err(why) = m.channel_id.pin(http, m) {
                println!("Could not pin message: {:?}", why);
            }
        }

        // fill bet state data array
        let size = g.len();
        let b = data.get_mut::<BetStateData>().unwrap();
        for i in 0..size {
            b.insert(i, states[i]);
        }
    }
}
