use std::collections::HashMap;
use crate::bot::consts::DISCORD_GUILD_ID;
use crate::bot::consts::DISCORD_ROLE_ANIMATEUR;
use crate::bot::consts::DISCORD_ROLE_TEAM_CODEUR;
use crate::bot::consts::DISCORD_ROLE_MODERATION;
use crate::bot::consts::DISCORD_EMBED_COLOR;
use crate::strings::*;
use crate::locale;

use fulgurobot_db::*;
use serenity::{
    client::Context,
    framework::standard::{ Args, CommandResult, macros::command },
    http::Http,
    model::channel::Message,
    utils::MessageBuilder,
};
use super::{
    core::GameData,
    BetState, BetStateData
};

fn send_message(message: &Message, http: &Http, reply: &str) {
    if let Err(why) = message.channel_id.send_message(http, |m| {
        m.embed(|e| {
            e.title(reply)
             .color(DISCORD_EMBED_COLOR);
            e
        });
        m
    }) {
        println!("Could not send message: {:?}", why);
    }
}

fn send_with_mention(message: &Message, http: &Http, reply: &str) {
    let user = http.get_member(DISCORD_GUILD_ID, message.author.id.0).unwrap();
    if let Err(why) = message.channel_id.send_message(http, |m| {
        m.embed(|e| {
            e.title(reply)
             .description(user)
             .color(DISCORD_EMBED_COLOR);
            e
        });
        m
    }) {
        println!("Could not send message: {:?}", why);
    }
}

fn bet_on_color(color: String,
                l: &HashMap<&str, &str>,
                context: &mut serenity::prelude::Context,
                message: &Message ,
                mut args: serenity::framework::standard::Args) {

    let mut args_ok : bool = true;
    let game_id = args.single::<usize>().unwrap_or_else(|_| {
        args_ok = false; 0
    });
    let nb_coq = args.single::<i32>().unwrap_or_else(|_| {
        args_ok = false; 0
    });
    if !args_ok {
        send_with_mention(message, &context.http, &format!("{}{}{}", locale!(l, "bet_0"), &color, locale!(l, "bet_1")));
        return
    }

    if nb_coq <= 0 {
        send_with_mention(message, &context.http, locale!(l, "bet_2"));
        return
    }

    let data = context.data.read();
    if let Some(game) = data.get::<BetStateData>().unwrap().get(&game_id) {
        match game {
            BetState::NotBetting    => {
                send_with_mention(message, &context.http, locale!(l, "bet_3"));
                return
            },
            BetState::WaitingResult => {
                send_with_mention(message, &context.http, locale!(l, "bet_4"));
                return
            },
            _ => ()
        }
    }

    let conn = connect_db();
    let id = message.author.id.0.to_string();

    if !user_exists(id.clone(), &conn) {
        let user = context.http.get_member(DISCORD_GUILD_ID, message.author.id.0).unwrap();
        create_user(id.clone(), user.nick.unwrap_or_else(|| message.author.name.clone()), &conn);
    }
    let games = data.get::<GameData>().unwrap();
    if game_id >= games.len() || games[game_id].is_none() {
        send_with_mention(message, &context.http, locale!(l, "bet_5"));
        return
    }
    let black = data.get::<GameData>().unwrap()[game_id].as_ref().unwrap().0.clone();
    let white = data.get::<GameData>().unwrap()[game_id].as_ref().unwrap().1.clone();

    // check if user has enough coq
    let mut coq = get_coq_of_user(id.clone(), &conn);
    // if user has already bet, give him back the amount bet to bet again
    if let Some(bet) = get_bet(id.clone(), black.clone(), white.clone(), &conn) {
        coq += bet.bet;
    }
    if coq - nb_coq >= 0 {
        create_bet(id, black, white, nb_coq, color, &conn);
    } else {
        send_with_mention(message, &context.http, locale!(l, "bet_6"));
    }
}


fn fulgurobot_worker(l: &HashMap<&str, &str>, context: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.title(locale!(l, "fulgurobot_0"))
             .color(DISCORD_EMBED_COLOR)
             .description(locale!(l, "fulgurobot_1"))
             .field(locale!(l, "fulgurobot_2"), locale!(l, "fulgurobot_3"), false)
             .field(locale!(l, "fulgurobot_4"), locale!(l, "fulgurobot_5"), false)
             .field(locale!(l, "fulgurobot_6"), locale!(l, "fulgurobot_7"), false)
             .field(locale!(l, "fulgurobot_8"), locale!(l, "fulgurobot_9"), false)
             .field(locale!(l, "fulgurobot_10"), locale!(l, "fulgurobot_11"), false);
            e
        });
        m
    }).expect("Could not send embed message");

    if message.author.has_role(&context, message.guild_id.unwrap(), DISCORD_ROLE_ANIMATEUR).unwrap() ||
    message.author.has_role(&context, message.guild_id.unwrap(), DISCORD_ROLE_MODERATION).unwrap() ||
    message.author.has_role(&context, message.guild_id.unwrap(), DISCORD_ROLE_TEAM_CODEUR).unwrap() {
        if let Err(why) = message.author.direct_message(&context, |m| {
            m.embed(|e| {
                e.title("Commandes pour controler Fulgurobot")
                 .color(DISCORD_EMBED_COLOR)
                 .field("!create_game noir blanc", "créé une partie pour parier et lui donne un id pour les autre commandes.", false)
                 .field("!debut_paris game_id", "démarre les paris pour la partie identifié par game_id.", false)
                 .field("!fin_paris game_id", "bloque les paris pour la partie identifié par game_id.", false)
                 .field("!resultat game_id couleur", "indique la couleur gagnante de la partie identifié par game_id.", false);
                e
            });
            m
        }) {
            println!("error sending message: {:?}", why);
        }
    }
    Ok(())
}

#[command]
#[bucket = "basic"]
// !fulgurobot
pub fn fulgurobot(context: &mut Context, message: &Message) -> CommandResult {
    fulgurobot_worker(&FRENCH, context, message)
}

#[command]
#[bucket = "basic"]
pub fn help(context: &mut Context, message: &Message) -> CommandResult {
    fulgurobot_worker(&ENGLISH, context, message)
}

#[command]
#[bucket = "basic"]
pub fn commands(context: &mut Context, message: &Message) -> CommandResult {
    fulgurobot_worker(&ENGLISH, context, message)
}

// !noir game_id bet
#[command]
#[bucket = "basic"]
fn noir(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    bet_on_color("noir".to_string(), &FRENCH, context, message, args);
    Ok(())
}

// !blanc game_id bet
#[command]
#[bucket = "basic"]
fn blanc(context: &mut Context, message: &Message, args: Args) -> CommandResult{
    bet_on_color("blanc".to_string(), &FRENCH, context, message, args);
    Ok(())
}

// !noir game_id bet
#[command]
#[bucket = "basic"]
fn black(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    bet_on_color("noir".to_string(), &ENGLISH, context, message, args);
    Ok(())
}

// !blanc game_id bet
#[command]
#[bucket = "basic"]
fn white(context: &mut Context, message: &Message, args: Args) -> CommandResult{
    bet_on_color("blanc".to_string(), &ENGLISH, context, message, args);
    Ok(())
}

// !create_game black white
#[command]
#[bucket = "basic"]
fn create_game(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut args_ok: bool = true;
    let black = args.single::<String>().unwrap_or_else(|_| {
        args_ok = false; "".into()
    });
    let white = args.single::<String>().unwrap_or_else(|_| {
        args_ok = false; "".into()
    });

    if !args_ok {
        send_with_mention(message, &context.http, "usage: !create_game noir blanc");
        return Ok(())
    }

    {
        let conn = connect_db();
        fulgurobot_db::create_game(black.clone(), white.clone(), &conn);
    }

    let mut data = context.data.write();
    let games = data.get_mut::<GameData>().unwrap();
    let mut index = games.len();
    for (i, e) in games.iter().enumerate() {
        if e.is_none()  {
            index = i;
            break;
        }
    }

    let reply = MessageBuilder::new()
                .push(format!("La partie de **{}** vs **{}** a été créée avec l'id : ", black, white))
                .push_bold_safe(format!("{}.", index))
                .build();
    let m = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.title(&reply)
             .color(DISCORD_EMBED_COLOR);
            e
        });
        m
    });
    if let Err(why) = m {
        println!("Could not send message: {:?}", why);
        return Ok(());
    }
    let m = m.unwrap();
    if let Err(why) = m.channel_id.pin(&context.http, &m) {
        println!("Could not pin message: {:?}", why);
    }
    if index != games.len() {
        games[index] = Some((black.clone(), white.clone(), Some(m)));
    } else {
        games.push(Some((black.clone(), white.clone(), Some(m))));
    }
    let state = data.get_mut::<BetStateData>().unwrap();
    state.insert(index, BetState::NotBetting);
    Ok(())
}

// !debut_paris game_id [timeout time]
#[command]
#[bucket = "basic"]
fn debut_paris(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut args_ok : bool = true;
    let game_id = args.single::<usize>().unwrap_or_else(|_| {
        args_ok = false; 0
    });
    let timeout = match args.single::<String>() {
        Ok(s) => {
            if s == "timeout".to_string() {
                Some(())
            } else {
                None
            }
        },
        Err(_) => None,
    };
    let mut time = 0;
    if timeout.is_some() {
        time = args.single::<u64>().unwrap_or_else(|_| {
            args_ok = false; 0
        });
    }

    if !args_ok {
        send_with_mention(message, &context.http, "Usage: !debut_paris game_id");
        return Ok(())
    }


    let mut data = context.data.write();
    if let Some(game) = data.get::<GameData>().unwrap()[game_id].as_ref() {
        let game = game.clone();
        let state = data.get_mut::<BetStateData>().unwrap().get_mut(&game_id).unwrap();
        if state == &BetState::NotBetting {
            *state = BetState::Betting;
            let conn = connect_db();
            update_game_state(game.0.clone(), game.1.clone(), BetState::Betting.into(), &conn);
            if timeout.is_some() { // launch a thread that stops betting after the timeout
                let cx = context.data.clone();
                let m = message.channel_id;
                let h = context.http.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(time * 60));
                    let mut cx = cx.write();
                    let state = cx.get_mut::<BetStateData>().unwrap().get_mut(&game_id).unwrap();
                    *state = BetState::WaitingResult;
                    let reply = MessageBuilder::new()
                        .push("Les paris de la partie ")
                        .push_bold_safe(format!("{}", game.0))
                        .push(" vs ")
                        .push_bold_safe(format!("{}", game.1))
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
            }
        } else if state == &BetState::Betting {
            send_with_mention(message, &context.http, "Les paris sont déjà en cours !");
            return Ok(())
        } else {
            send_with_mention(message, &context.http, "La partie est en attente du résultat");
            return Ok(())
        }
        let reply = MessageBuilder::new()
                    .push("Les paris sont ouverts !")
                    .build();
        send_message(message, &context.http, &reply);
    } else {
        send_with_mention(message, &context.http, "Mauvais id de partie");
    }
    Ok(())
}

#[command]
#[bucket = "basic"]
// !fin_paris game_id
fn fin_paris(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut arg_ok : bool = true;
    let game_id = args.single::<usize>().unwrap_or_else(|_| {
        arg_ok = false; 0
    });
    if !arg_ok {
        send_with_mention(message, &context.http, "Usage: !fin_paris game_id");
        return Ok(())
    }

    let mut data = context.data.write();
    {
        let game = data.get::<GameData>().unwrap();
        if game_id >= game.len() || game[game_id].is_none() {
            send_with_mention(message, &context.http, "Mauvais id de partie");
        }
        let black = &game[game_id].as_ref().unwrap().0;
        let white = &game[game_id].as_ref().unwrap().1;
        {
            let conn = connect_db();
            update_game_state(black.clone(), white.clone(), BetState::WaitingResult.into(), &conn);
            let game = fulgurobot_db::get_game(black.clone(), white.clone(), &conn).unwrap();
            let reply = MessageBuilder::new()
            .push("Les paris de la partie ")
            .push_bold_safe(format!("{}", black))
            .push(" vs ")
            .push_bold_safe(format!("{}", white))
            .push(" sont finis ! \nTotal pour ")
            .push_bold_safe(format!("{}", black))
            .push(format!(" (noir) : {} coquillages\nTotal pour ", game.black_bet))
            .push_bold_safe(format!("{}", white))
            .push(format!(" (blanc) : {} coquillages", game.white_bet))
            .build();
            send_message(message, &context.http, &reply);
        }
    }

    let state = data.get_mut::<BetStateData>().unwrap().get_mut(&game_id).unwrap();
    *state = BetState::WaitingResult;


    Ok(())
}

#[command]
#[bucket = "basic"]
// !resultat game_id color
fn resultat(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut args_ok : bool = true;
    let game_id = args.single::<usize>().unwrap_or_else(|_| {
        args_ok = false; 0
    });
    let color = args.single::<String>().unwrap_or_else (|_| {
        args_ok = false; "".into()
    });
    if color != "noir".to_owned() && color != "blanc".to_owned() {
        send_with_mention(message, &context.http, "Usage: !resultat game_id couleur (blanc ou noir)");
        return Ok(())
    }

    if !args_ok {
        send_with_mention(message, &context.http, "Usage: !resultat game_id couleur");
        return Ok(())
    }

    let mut data = context.data.write();

    {
        let games = data.get_mut::<GameData>().unwrap();
        if game_id >= games.len() || games[game_id].is_none() {
            send_with_mention(message, &context.http, "Mauvais id de partie");
            return Ok(())
        }
    }
    let state = data.get::<BetStateData>().unwrap().get(&game_id).unwrap();
    if state != &BetState::WaitingResult {
        send_with_mention(message, &context.http, "Les paris n'ont même pas encore commencé !");
        return Ok(())
    }

    let games = data.get_mut::<GameData>().unwrap();

    let conn = connect_db();
    let (black, white, pin) = &games[game_id].as_ref().unwrap();
    let pin = pin.clone();
    let game = get_game(black.clone(), white.clone(), &conn).unwrap();

    let users = match get_users_bet_color(black.clone(), white.clone(), color.clone(), &conn) {
        Some(u) => u,
        None => Vec::new(), //maybe change later to cancel the bets if there is no winner
    };
    let total_color =
    match color.as_str() {
        "noir" => {
            game.black_bet
        },
        "blanc" => {
            game.white_bet
        },
        _ => 0,
    };
    let total = game.black_bet + game.white_bet;

    let mut reply = MessageBuilder::new();
    if let Err(why) = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.color(DISCORD_EMBED_COLOR)
             .title("Gagnants :");
            for user in users {
                let bet = get_bet(user.id.clone(), black.clone(), white.clone(), &conn).unwrap();
                let percent = bet.bet as f32 / total_color as f32;
                let gain = (total as f32 * percent).ceil() as i32;
                add_coq_to_user(user.id.clone(), gain, &conn);

                let user = context.http.get_member(DISCORD_GUILD_ID, user.id.parse().unwrap()).unwrap();
                reply.push_bold_safe(user)
                .push(format!(" a gagné **{}** coquillages !\n", gain));
            }
            let reply = reply.build();
            if reply.as_str() != "" {
                e.description(&reply);
            }
            else {
                e.description("Il n'y a aucun gagnants !");
            }

            e
        });
        m
    }) {
        println!("Couldn't send embed: {:?}", why);
    }

    remove_bets_of_game(black.clone(), white.clone(), &conn);
    delete_game(black.clone(), white.clone(), &conn);
    games[game_id] = None;

    // if game has a pinned message, unpin it
    if pin.is_some() {
        if let Err(why) = message.channel_id.unpin(&context.http, pin.as_ref().unwrap()) {
            println!("Couldn't unpin message : {:?}", why);
        }
    } else {
        // check if there is no more game running
        let mut is_none = true;
        for g in &*games {
            if g.is_some() {
                is_none = false;
                break
            }
        }
        // if no more games are running, delete all pins
        // (it deletes for example restored context pin)
        if is_none {
            for p in message.channel_id.pins(&context.http).unwrap() {
                if let Err(why) = message.channel_id.unpin(&context.http, p) {
                    println!("Could not unpin message: {:?}", why);
                }
            }
        }
    }
    data.get_mut::<BetStateData>().unwrap().remove(&game_id);

    Ok(())
}

fn coq_worker(l: &HashMap<&str, &str>, context: &mut Context, message: &Message) -> CommandResult {
    let id = message.author.id.to_string();

    let conn = connect_db();
    if !user_exists(id.clone(), &conn) {
        let user = context.http.get_member(DISCORD_GUILD_ID, message.author.id.0).unwrap();
        create_user(id.clone(), user.nick.unwrap_or_else(|| message.author.name.clone()), &conn);
    }

    let coq = get_coq_of_user(id, &conn);

    let reply = MessageBuilder::new()
                .push_bold_safe(message.author.name.clone())
                .push(format!("{}**{}**{}", locale!(l, "coq_0"), coq, locale!(l, "coq_1")))
                .build();

    if let Err(why) = message.author.direct_message(&context, |m| {
            m.content(&reply)
    }) {
        println!("Could not send message: {:?}", why);
    }

    Ok(())
}

#[command]
#[bucket = "basic"]
// !coq
fn coq(context: &mut Context, message: &Message) -> CommandResult {
    coq_worker(&FRENCH, context, message)
}

#[command]
#[bucket = "basic"]
fn shell(context: &mut Context, message: &Message) -> CommandResult {
    coq_worker(&ENGLISH, context, message)
}

fn recharge_worker(l: &HashMap<&str, &str>, context: &mut Context, message: &Message) -> CommandResult {
    let conn = connect_db();

    // add user if he/she doesn't exists
    if !user_exists(message.author.id.to_string(), &conn) {
        let user = context.http.get_member(DISCORD_GUILD_ID, message.author.id.0).unwrap();
        create_user(message.author.id.to_string(), user.nick.unwrap_or_else(|| message.author.name.clone()), &conn);
    }

    // update_recharge_user check if user has enough recharge and use one (adds 200 coq to user)
    if let Ok(nb_recharge_left) = update_recharge_user(message.author.id.to_string(), -1, &conn) {
        //feedback
        let reply = MessageBuilder::new()
            .push(locale!(l, "recharge_0"))
            .push(format!("{}**{}**{}", locale!(l, "recharge_1"), nb_recharge_left, locale!(l, "recharge_2")))
            .build();
        send_with_mention(message, &context.http, &reply);
    } else {
        let reply = MessageBuilder::new()
            .push(locale!(l, "recharge_3"))
            .build();
        send_with_mention(message, &context.http, &reply);
    }
    Ok(())
}

#[command]
#[bucket = "basic"]
fn recharge(context: &mut Context, message: &Message) -> CommandResult {
    recharge_worker(&FRENCH, context, message)
}

#[command]
#[bucket = "basic"]
fn refill(context: &mut Context, message: &Message) -> CommandResult {
    recharge_worker(&ENGLISH, context, message)
}

fn nb_recharge_worker(l: &HashMap<&str, &str>, context: &mut Context, message: &Message) -> CommandResult {
    let conn = connect_db();
    // add user if he/she doesn't exists
    if !user_exists(message.author.id.to_string(), &conn) {
        let user = context.http.get_member(DISCORD_GUILD_ID, message.author.id.0).unwrap();
        create_user(message.author.id.to_string(), user.nick.unwrap_or_else(|| message.author.name.clone()), &conn);
    }

    let nb_recharge = match get_recharge_user(message.author.id.to_string(), &conn) {
        Ok(n) => n,
        Err(e) => { println!("Error reading database: {:?}", e); return Ok(()) }
    };

    // feedback
    let reply = MessageBuilder::new()
        .push_bold_safe(&message.author)
        .push(format!("{}**{}**{}", locale!(l, "nb_recharge_0"), nb_recharge, locale!(l, "nb_recharge_1")))
        .build();
    if let Err(why) = message.author.direct_message(&context, |m| {
        m.content(&reply)
    }) {
        println!("Couldn't send message {:?}", why);
    }
    Ok(())
}

#[command]
#[bucket = "basic"]
fn nb_recharge(context: &mut Context, message: &Message) -> CommandResult {
    nb_recharge_worker(&FRENCH, context, message)
}

#[command]
#[bucket = "basic"]
fn nb_refill(context: &mut Context, message: &Message) -> CommandResult {
    nb_recharge_worker(&ENGLISH, context, message)
}

#[command]
#[bucket = "basic"]
// !give user_id nb_coq
fn give(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    // checking args are correct
    let mut args_ok = true;
    let _ = args.single::<String>().unwrap_or_else(|_| { // we discard the name here because we get the id through message.mentions()
        args_ok = false; "".to_owned()
    });
    let nb_coq = args.single::<i32>().unwrap_or_else(|_| {
        args_ok = false; 0
    });
    if !args_ok || nb_coq <= 0  {
        send_with_mention(message, &context.http, "Usage: !give @name nb_coq (> 0)");
        return Ok(())
    } else if nb_coq > 2000 { // limite de 2000 coquillages
        send_with_mention(message, &context.http, "Impossible de donner plus de 2000 coquillages !");
        return Ok(())
    }

    //retrieving user to give coq to
    let id = message.mentions.first().unwrap();
    let id_s = id.to_string();

    let conn = connect_db();
    if !user_exists(message.author.id.to_string(), &conn) {
        let user = context.http.get_member(DISCORD_GUILD_ID, message.author.id.0).unwrap();
        create_user(message.author.id.to_string(), user.nick.unwrap_or_else(|| message.author.name.clone()), &conn);
    }
    // if user to give coq to doesn't exit cancel operation
    if !user_exists(id_s.clone(), &conn) {
        send_with_mention(message, &context.http, &format!("L'utilisateur {} n'est pas dans la base de données du bot !", id));
        return Ok(())
    }

    if let Err(_) = trade_coq(message.author.id.to_string(), id_s, nb_coq, &conn) {
        send_message(message, &context.http, "Erreur pendant l'échange de coquillages. L'échange est annulé");
    } else {
        // feedback
        let reply = MessageBuilder::new()
            .push_bold_safe(format!("{}", message.author))
            .push(format!(" a donné {} coquillages à ", nb_coq))
            .push_bold_safe(format!("{}", id))
            .build();
        send_message(message, &context.http, &reply);
    }

    Ok(())
}

fn etat_worker(l: &HashMap<&str, &str>, context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut arg_ok = true;
    let id = args.single::<usize>().unwrap_or_else(|_| {
        arg_ok = false; 0
    });
    if !arg_ok {
        send_with_mention(message, &context.http, locale!(l, "etat_0"));
        return Ok(())
    }

    let data = context.data.read();
    let state = match data.get::<BetStateData>().unwrap().get(&id) {
        Some(state) => state,
        None => {
            send_with_mention(message, &context.http, locale!(l, "etat_1"));
            return Ok(())
        },
    };

    message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
        e.color(DISCORD_EMBED_COLOR);
            match state {
                BetState::Betting => e.title(locale!(l, "etat_2")),
                BetState::WaitingResult => e.title(locale!(l, "etat_3")),
                BetState::NotBetting => {
                    e.title(locale!(l, "etat_4"));
                    return e
                }
            };
            let game = match data.get::<GameData>().unwrap()[id].as_ref() {
                Some(g) => g,
                None => { println!("aaah"); return e },
            };
            let conn = connect_db();
            let game = get_game(game.0.clone(), game.1.clone(), &conn).unwrap();
            e.description(format!("{}**{}**{}{}{}**{}**{}{}\n",
                locale!(l, "etat_5"), &game.black, locale!(l, "etat_6"), game.black_bet, locale!(l, "etat_7"), &game.white, locale!(l, "etat_8"), game.white_bet));

            let v1 = fulgurobot_db::get_bets_color(game.black.clone(), game.white.clone(), "noir".to_string(), 10, &conn);
            if !v1.is_empty() {
                let mut string = "".to_owned();
                for b in v1 {
                    string.push_str(&format!("{}\n", b));
                }
                e.field(locale!(l, "etat_9"), string, true);
            }

            let v2 = fulgurobot_db::get_bets_color(game.black, game.white, "blanc".to_string(), 10, &conn);
            if !v2.is_empty() {
                let mut string = "".to_owned();
                for b in v2 {
                    string.push_str(&format!("{}\n", b));
                }
                e.field(locale!(l, "etat_10"), string, true);
            }

            e
        });
        m
    }).expect("Could not send embed");
    Ok(())
}

#[command]
#[bucket = "basic"]
// !etat id
fn etat(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    etat_worker(&FRENCH, context, message, args)
}

#[command]
#[bucket = "basic"]
// !etat id
fn state(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    etat_worker(&ENGLISH, context, message, args)
}

#[command]
#[bucket = "basic"]
// !boost @user
fn boost(context: &mut Context, message: &Message) -> CommandResult {
    let user = message.mentions.first().unwrap();
    let id = user.id.to_string();
    let conn = connect_db();

    // add user if he/she doesn't exists
    if !user_exists(id.clone(), &conn) {
        let user = context.http.get_member(DISCORD_GUILD_ID, message.author.id.0).unwrap();
        create_user(id.clone(), user.nick.unwrap_or_else(|| message.author.name.clone()), &conn);
    }

    boost_user(id, &conn);
    let reply = MessageBuilder::new()
        .push_bold_safe(&user)
        .push("Tu as gagné 200 coquillages !\n")
        .build();
    send_with_mention(message, &context.http, &reply);
    Ok(())
}
