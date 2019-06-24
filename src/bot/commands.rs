use serenity::framework::standard::Args;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use crate::bot::core::GameData;
use serenity::utils::MessageBuilder;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::command;
use fulgurobot_db::*;
use super::{BetState, BetStateData};


fn bet_on_color(color: String,
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
        if let Err(why) = message.channel_id.say(&context.http, format!("Usage: !{} game_id nb_coq", &color)) {
            println!("Could not send message: {:?}", why);
        }
        return
    }

    let data = context.data.read();
    if let Some(game) = data.get::<BetStateData>().unwrap().get(&game_id) {
        match game {
            BetState::NotBetting    => {
                if let Err(why) = message.channel_id.say(&context.http, "Les paris n'ont pas démarré.") {
                    println!("Could not send message: {:?}", why);
                }
                return
            },
            BetState::WaitingResult => {
                if let Err(why) = message.channel_id.say(&context.http, "Les paris sont finis !") {
                    println!("Could not send message: {:?}", why);
                }
                return
            },
            _ => ()
        }
    }

    let conn = connect_db();
    let id = message.author.id.0.to_string();

    if !user_exists(id.clone(), &conn) {
        create_user(id.clone(), message.author.name.clone(), &conn);
    }
    let games = data.get::<GameData>().unwrap();
    if game_id >= games.len() || games[game_id] == None {
        let reply = MessageBuilder::new()
                    .push_bold_safe(message.author.name.clone())
                    .push(", Cette partie n'existe pas.")
                    .build();
        if let Err(why) = message.channel_id.say(&context.http, &reply) {
            println!("Could not send message: {:?}", why);
        }
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
    if coq - nb_coq > 0 {
        create_bet(id, black, white, nb_coq, color, &conn);
    } else {
        let reply = MessageBuilder::new()
                    .push_bold_safe(message.author.name.clone())
                    .push(", Tu n'as pas assez de coquillages.")
                    .build();
        if let Err(why) = message.channel_id.say(&context.http, &reply) {
            println!("Could not send message: {:?}", why);
        }
    }
}


#[command]
// !fulgurobot
pub fn fulgurobot(context: &mut Context, message: &Message) -> CommandResult {
    let mut reply = MessageBuilder::new();
    reply.push("Commandes pour parier :\n!noir i x -> parie x coquillages sur noir pour la partie i\n!blanc i x -> parie x coquillages sur blanc pour la partie i\n!coq -> envoie en message privé votre nombre de coquillages");
    if message.author.has_role(&context, message.guild_id.unwrap(), 400_904_374_219_571_201).unwrap() ||
        message.author.has_role(&context, message.guild_id.unwrap(), 291_868_975_015_657_472).unwrap() ||
        message.author.has_role(&context, message.guild_id.unwrap(), 416_986_920_829_321_228).unwrap() {
            reply.push("\n!create_game noir blanc -> créé une partie pour parier et lui donne un id pour les autre commandes.
                        !debut_paris game_id -> démarre les paris pour la partie identifié par game_id.
                        !fin_paris game_id -> bloque les paris pour la partie identifié par game_id.
                        !resultat game_id couleur -> indique la couleur gagnante de la partie identifié par game_id.");
    }

    let reply = reply.build();

    if let Err(why) = message.author.direct_message(&context, |m| { m.content(&reply) }) {
        println!("error sending message: {:?}", why);
    }
    Ok(())
}

// !noir game_id bet
#[command]
fn noir(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    bet_on_color("noir".to_string(), context, message, args);
    Ok(())
}

// !blanc game_id bet
#[command]
fn blanc(context: &mut Context, message: &Message, args: Args) -> CommandResult{
    bet_on_color("blanc".to_string(), context, message, args);
    Ok(())
}

// !create_game black white
#[command]
fn create_game(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut args_ok: bool = true;
    let black = args.single::<String>().unwrap_or_else(|_| {
        args_ok = false; "".into()
    });
    let white = args.single::<String>().unwrap_or_else(|_| {
        args_ok = false; "".into()
    });
    if !args_ok {
        if let Err(why) = message.channel_id.say(&context.http, "Usage: !create_game noir blanc") {
            println!("Could not send message: {:?}", why);
        }
    }

    {
        let conn = connect_db();
        fulgurobot_db::create_game(black.clone(), white.clone(), &conn);
    }

    let mut data = context.data.write();
    let games = data.get_mut::<GameData>().unwrap();
    let mut index = games.len();
    for (i, e) in games.iter().enumerate() {
        if *e == None {
            index = i;
            break;
        }
    }
    if index != games.len() {
        games[index] = Some((black.clone(), white.clone()));
    } else {
        games.push(Some((black.clone(), white.clone())));
    }
    let state = data.get_mut::<BetStateData>().unwrap();
    state.insert(index, BetState::NotBetting);

    let reply = MessageBuilder::new()
                .push(format!("La partie de {} vs {} a été créée avec l'id : ", black, white))
                .push_bold_safe(format!("{}.", index))
                .build();
    let m = message.channel_id.say(&context.http, &reply);
    if let Err(why) = m {
        println!("Could not send message: {:?}", why);
        return Ok(());
    }
    let m = m.unwrap();
    if let Err(why) = m.channel_id.pin(&context.http, &m) {
        println!("Could not pin message: {:?}", why);
    }
    Ok(())
}

// !debut_paris game_id
#[command]
fn debut_paris(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut arg_ok : bool = true;
    let game_id = args.single::<usize>().unwrap_or_else(|_| {
        arg_ok = false; 0
    });

    if !arg_ok {
        if let Err(why) = message.channel_id.say(&context.http, "Usage: !debut_paris game_id") {
            println!("Could not send message: {:?}", why);
        }
    }

    let mut data = context.data.write();
    if let Some(_game) = data.get::<GameData>().unwrap()[game_id].as_ref() {
        let state = data.get_mut::<BetStateData>().unwrap().get_mut(&game_id).unwrap();
        if state == &BetState::NotBetting {
            *state = BetState::Betting;
        } else {
            let reply = MessageBuilder::new()
                        .push("Impossible de commencer les paris.\n(En attente de résultat ou paris déjà en cours)")
                        .build();
            if let Err(why) = message.channel_id.say(&context.http, &reply) {
                println!("Could not send message: {:?}", why);
            }
            return Ok(());
        }
        let reply = MessageBuilder::new()
                    .push("Les paris sont ouverts !")
                    .build();
        if let Err(why) = message.channel_id.say(&context.http, &reply) {
            println!("Could not send message: {:?}", why);
        }
    } else {
        let reply = MessageBuilder::new()
                    .push("Mauvais id de partie")
                    .build();
        if let Err(why) = message.channel_id.say(&context.http, &reply) {
            println!("Could not send message: {:?}", why);
        }
    }
    Ok(())
}

#[command]
// !fin_paris game_id
fn fin_paris(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut arg_ok : bool = true;
    let game_id = args.single::<usize>().unwrap_or_else(|_| {
        arg_ok = false; 0
    });
    if !arg_ok {
        if let Err(why) = message.channel_id.say(&context.http, "Usage: !fin_paris game_id") {
            println!("Could not send message: {:?}", why);
        }
    }

    let mut data = context.data.write();
    {
        let game = data.get::<GameData>().unwrap();
        if game_id >= game.len() || game[game_id] == None {
            let reply = MessageBuilder::new()
                        .push("Mauvais id de partie")
                        .build();
            if let Err(why) = message.channel_id.say(&context.http, &reply) {
                println!("Could not send message: {:?}", why);
            }
        }
        let black = &game[game_id].as_ref().unwrap().0;
        let white = &game[game_id].as_ref().unwrap().1;
        {
            let conn = connect_db();
            let game = fulgurobot_db::get_game(black.clone(), white.clone(), &conn).unwrap();
            let reply = MessageBuilder::new()
            .push(format!("Les paris de la partie {} vs {} sont finis ! \
                            \nTotal pour {} : {} coquillages \
                            \nTotal pour {} : {} coquillages", black, white, black, game.black_bet, white, game.white_bet))
            .build();
            if let Err(why) = message.channel_id.say(&context.http, &reply) {
                println!("Could not send message: {:?}", why);
            }
        }
    }

    let state = data.get_mut::<BetStateData>().unwrap().get_mut(&game_id).unwrap();
    *state = BetState::WaitingResult;


    Ok(())
}

#[command]
// !resultat game_id color
fn resultat(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut args_ok : bool = true;
    let game_id = args.single::<usize>().unwrap_or_else(|_| {
        args_ok = false; 0
    });
    let color = args.single::<String>().unwrap_or_else (|_| {
        args_ok = false; "".into()
    });
    if color != "noir" || color != "blanc" {
        if let Err(why) = message.channel_id.say(&context.http, "Usage: !resultat game_id couleur (blanc ou noir)") {
            println!("Could not send message: {:?}", why);
        }
        return Ok(());
    }

    if !args_ok {
        if let Err(why) = message.channel_id.say(&context.http, "Usage: !resultat game_id couleur") {
            println!("Could not send message: {:?}", why);
        }
    }

    let mut data = context.data.write();

    {
        let games = data.get_mut::<GameData>().unwrap();
        if game_id >= games.len() || games[game_id] == None {
            let reply = MessageBuilder::new()
                        .push("Mauvais id de partie")
                        .build();
            if let Err(why) = message.channel_id.say(&context.http, &reply) {
                println!("Could not send message: {:?}", why);
            }
        }
    }
    let state = data.get::<BetStateData>().unwrap().get(&game_id).unwrap();
    if state != &BetState::WaitingResult {
        return Ok(());
    }

    let games = data.get_mut::<GameData>().unwrap();

    let conn = connect_db();
    let black = &games[game_id].as_ref().unwrap().0;
    let white = &games[game_id].as_ref().unwrap().1;
    let game = get_game(black.clone(), white.clone(), &conn).unwrap();

    let users = match get_users_bet_color(black.clone(), white.clone(), color.clone(), &conn) {
        Some(u) => u,
        None => Vec::new(), //maybe change later to cancel the bets if there is no winner
    };
    let mut total = 0;
    match color.as_str() {
        "noir" => {
            total = game.black_bet;
        },
        "blanc" => {
            total = game.white_bet;
        },
        _ => (),
    }
    let mut reply = MessageBuilder::new();
    for user in users {
        let bet = get_bet(user.id.clone(), black.clone(), white.clone(), &conn).unwrap();
        let percent = bet.bet / total;
        let gain = total * percent;
        add_coq_to_user(user.id, gain, &conn);


        reply.push_bold_safe(user.name)
            .push(format!(" a gagné {} coquillages !\n", gain));
    }
    let reply = reply.build();
    if reply.as_str() != "" {
        if let Err(why) = message.channel_id.say(&context.http, &reply) {
            println!("Could not send message: {:?}", why);
        }
    }
    else if let Err(why) = message.channel_id.say(&context.http, "Il n'y a aucun gagnants !") {
        println!("Could not send message: {:?}", why);
    }
    remove_bets_of_game(black.clone(), white.clone(), &conn);
    delete_game(black.clone(), white.clone(), &conn);
    games[game_id] = None;
    data.get_mut::<BetStateData>().unwrap().remove(&game_id);

    Ok(())
}

#[command]
// !coq
fn coq(context: &mut Context, message: &Message) -> CommandResult {
    let id = message.author.id.to_string();

    let conn = connect_db();
    if !user_exists(id.clone(), &conn) {
        create_user(id.clone(), message.author.name.clone(), &conn);
    }

    let coq = get_coq_of_user(id, &conn);

    let reply = MessageBuilder::new()
                .push_bold_safe(message.author.name.clone())
                .push(format!(", vous avez {} coquillages.", coq))
                .build();

    if let Err(why) = message.author.direct_message(&context, |m| {
            m.content(&reply)
    }) {
        println!("Could not send message: {:?}", why);
    }

    Ok(())
}
