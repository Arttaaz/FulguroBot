use fulgurobot_db::connect_db;
use serenity::model::id::ChannelId;
use fulgurobot_db::update_game_state;
use chrono::prelude::{ DateTime, Utc };
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
use crate::bot::consts::DISCORD_CHANNEL_DEBUG_ID;
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

impl EventHandler for Handler {
    fn ready(&self, ctx: serenity::prelude::Context, _: serenity::model::gateway::Ready) {
        ctx.set_activity(serenity::model::gateway::Activity::playing("battre AlphaZero"));
        println!("FulguroBot is ready.")
    }
}

group!({
    name: "general",
    options: {},
    commands: [noir, black, blanc, white, fulgurobot, help, commands, coq, shell, nb_recharge, nb_refill, recharge, refill, etat, state, /*give*/]
});

group!({
    name: "control",
    options: { allowed_roles: ["Animateur", "Team Codeur", "Modération", "Admin FulguroGo"] },
    commands: [create_game, debut_paris, annuler, fin_paris, resultat],
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
    let chan_debug_id = serenity::model::id::ChannelId(DISCORD_CHANNEL_DEBUG_ID);

    client.with_framework(StandardFramework::new()
                            .bucket("basic", |b| b.delay(2).time_span(10).limit(3))
                            .bucket("give", |b| b.time_span(10).limit(1))
                            .configure(|c| c.prefix("!")
                                            .allow_dm(false)
                                            .ignore_bots(false)
                                            .allowed_channels(vec![chan_id, chan_debug_id].into_iter().collect()))
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
        dbg!("nani");

        // fill GameData array
        for (i, game) in games.iter().enumerate() {
            g.push(Some((game.black.clone(), game.white.clone(), None)));
            if !game.start.is_empty() {
                let start = chrono::NaiveDateTime::parse_from_str(&game.start,
                    "%Y-%m-%d %H:%M:%S%.9f UTC").expect("Wat");
                let start = DateTime::from_utc(start, chrono::offset::Utc);
                if Utc::now() > start + chrono::Duration::seconds((game.timeout) as i64) {
                    states.push(BetState::WaitingResult);
                } else if Utc::now() <= start + chrono::Duration::seconds((game.timeout) as i64) {
                    states.push(BetState::Betting);
                    let black = game.black.clone();
                    let white = game.white.clone();
                    let d2 = client.data.clone();
                    let h = client.cache_and_http.http.clone();
                    let m = ChannelId(DISCORD_CHANNEL_ID);
                    let g2 = game.clone();
                    let i = i.clone();
                    let s = start.clone();
                    dbg!("what");
                    std::thread::spawn(move || {
                        dbg!(s + chrono::Duration::seconds((g2.timeout + 60) as i64) - Utc::now());
                        std::thread::sleep((s + chrono::Duration::seconds((g2.timeout + 60) as i64) - Utc::now()).to_std().expect("what"));
                        let mut cx = d2.write();
                        let state = cx.get_mut::<BetStateData>().unwrap().get_mut(&i).unwrap();
                        *state = BetState::WaitingResult;
                        let conn = connect_db();
                        update_game_state(black.clone(), white.clone(), BetState::WaitingResult.into(), &conn);
                        let reply = MessageBuilder::new()
                            .push("Les paris de la partie ")
                            .push_bold_safe(format!("{}", black))
                            .push(" vs ")
                            .push_bold_safe(format!("{}", white))
                            .push(" sont finis !")
                            .build();
                        if let Err(why) = m.send_message(&h, |m| {
                            m.embed(|e| {
                                e.title(reply)
                                 .color(DISCORD_EMBED_COLOR);
                                e
                            });
                            m
                        }) {
                            println!("Could not send message: {:?}", why);
                        }
                    });
                } else {
                    states.push(game.state.into());
                }
            } else {
                states.push(game.state.into());
            }
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
