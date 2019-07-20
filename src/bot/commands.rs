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
    if let Err(why) = message.channel_id.say(http, reply) {
        println!("Could not send message: {:?}", why);
    }
}

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
        send_message(message, &context.http, &format!("Usage: !{} game_id nb_coq", &color));
        return
    }

    let data = context.data.read();
    if let Some(game) = data.get::<BetStateData>().unwrap().get(&game_id) {
        match game {
            BetState::NotBetting    => {
                send_message(message, &context.http, "Les paris n'ont pas démarré.");
                return
            },
            BetState::WaitingResult => {
                send_message(message, &context.http, "Les paris sont finis !");
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
        send_message(message, &context.http, &reply);
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
        send_message(message, &context.http, &reply);
    }
}


#[command]
// !fulgurobot
pub fn fulgurobot(context: &mut Context, message: &Message) -> CommandResult {
    //change to embedded message
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
        send_message(message, &context.http, "Usage: !create_game noir blanc");
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
        send_message(message, &context.http, "Usage: !debut_paris game_id");
        return Ok(())
    }

    let mut data = context.data.write();
    if let Some(game) = data.get::<GameData>().unwrap()[game_id].as_ref() {
        let game = game.clone();
        let state = data.get_mut::<BetStateData>().unwrap().get_mut(&game_id).unwrap();
        if state == &BetState::NotBetting {
            *state = BetState::Betting;
            let conn = connect_db();
            update_game_state(game.0, game.1, BetState::Betting.into(), &conn);
        } else {
            let reply = MessageBuilder::new()
                        .push("Impossible de commencer les paris.\n(En attente de résultat ou paris déjà en cours)")
                        .build();
            send_message(message, &context.http, &reply);
            return Ok(())
        }
        let reply = MessageBuilder::new()
                    .push("Les paris sont ouverts !")
                    .build();
        send_message(message, &context.http, &reply);
    } else {
        let reply = MessageBuilder::new()
                    .push("Mauvais id de partie")
                    .build();
        send_message(message, &context.http, &reply);
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
        send_message(message, &context.http, "Usage: !fin_paris game_id");
        return Ok(())
    }

    let mut data = context.data.write();
    {
        let game = data.get::<GameData>().unwrap();
        if game_id >= game.len() || game[game_id] == None {
            let reply = MessageBuilder::new()
                        .push("Mauvais id de partie")
                        .build();
            send_message(message, &context.http, &reply);
        }
        let black = &game[game_id].as_ref().unwrap().0;
        let white = &game[game_id].as_ref().unwrap().1;
        {
            let conn = connect_db();
            update_game_state(black.clone(), white.clone(), BetState::WaitingResult.into(), &conn);
            let game = fulgurobot_db::get_game(black.clone(), white.clone(), &conn).unwrap();
            let reply = MessageBuilder::new()
            .push(format!("Les paris de la partie {} vs {} sont finis ! \
                            \nTotal pour {} : {} coquillages \
                            \nTotal pour {} : {} coquillages", black, white, black, game.black_bet, white, game.white_bet))
            .build();
            send_message(message, &context.http, &reply);
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
        send_message(message, &context.http, "Usage: !resultat game_id couleur (blanc ou noir)");
        return Ok(())
    }

    if !args_ok {
        send_message(message, &context.http, "Usage: !resultat game_id couleur");
        return Ok(())
    }

    let mut data = context.data.write();

    {
        let games = data.get_mut::<GameData>().unwrap();
        if game_id >= games.len() || games[game_id] == None {
            let reply = MessageBuilder::new()
                        .push("Mauvais id de partie")
                        .build();
            send_message(message, &context.http, &reply);
        }
    }
    let state = data.get::<BetStateData>().unwrap().get(&game_id).unwrap();
    if state != &BetState::WaitingResult {
        return Ok(())
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
        send_message(message, &context.http, &reply);
    }
    else {
        send_message(message, &context.http, "Il n'y a aucun gagnants !");
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

#[command]
fn state(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let id = args.single::<usize>().unwrap();

    let data = context.data.read();
    let state = data.get::<BetStateData>().unwrap().get(&id).unwrap();

    let reply = MessageBuilder::new()
                    .push(format!("State: {:?}", state))
                    .build();
    send_message(message, &context.http, &reply);

    Ok(())
}

#[command]
fn boost(context: &mut Context, message: &Message) -> CommandResult {
    let conn = connect_db();

    // add user if he/she doesn't exists
    if !user_exists(message.author.id.to_string(), &conn) {
        create_user(message.author.id.to_string(), message.author.name.clone(), &conn);
    }

    // update_boost_user check if user has enough boost and use one (adds 200 coq to user)
    if let Ok(nb_boost_left) = update_boost_user(message.author.id.to_string(), -1, &conn) {
        //feedback
        let reply = MessageBuilder::new()
            .push_bold_safe(&message.author.name)
            .push(", Tu as gagné 200 coquillages !\n")
            .push(format!("Il te reste {} boosts.", nb_boost_left))
            .build();
        send_message(message, &context.http, &reply);
    } else {
        let reply = MessageBuilder::new()
            .push_bold_safe(&message.author.name)
            .push(", Tu n'as plus de boosts !")
            .build();
        send_message(message, &context.http, &reply);
    }
    Ok(())
}

#[command]
fn nb_boost(context: &mut Context, message: &Message) -> CommandResult {
    let conn = connect_db();
    // add user if he/she doesn't exists
    if !user_exists(message.author.id.to_string(), &conn) {
        create_user(message.author.id.to_string(), message.author.name.clone(), &conn);
    }

    let nb_boost = match get_boost_user(message.author.id.to_string(), &conn) {
        Ok(n) => n,
        Err(e) => { println!("Error reading database: {:?}", e); return Ok(()) }
    };

    // feedback
    let reply = MessageBuilder::new()
        .push_bold_safe(&message.author.name)
        .push(format!(", Il te reste : {} boosts !", nb_boost))
        .build();
    if let Err(why) = message.author.direct_message(&context, |m| {
            m.content(&reply)
    }) {
        println!("Couldn't send message {:?}", why);
    }
    Ok(())
}

#[command]
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
        send_message(message, &context.http, "Usage: !give @name nb_coq (> 0)");
        return Ok(())
    } else if nb_coq > 2000 { // limite de 2000 coquillages
        send_message(message, &context.http, "Impossible de donner plus de 2000 coquillages !");
        return Ok(())
    }

    //retrieving user to give coq to
    let id = message.mentions.first().unwrap();
    let id_s = id.to_string();

    let conn = connect_db();
    if !user_exists(message.author.id.to_string(), &conn) {
        create_user(message.author.id.to_string(), message.author.name.clone(), &conn);
    }
    // if user to give coq to doesn't exit cancel operation
    if !user_exists(id_s.clone(), &conn) {
        send_message(message, &context.http, &format!("L'utilisateur {} n'est pas dans la base de données du bot !", id));
        return Ok(())
    }

    if let Err(_) = trade_coq(message.author.id.to_string(), id_s, nb_coq, &conn) {
        send_message(message, &context.http, "Erreur pendant l'échange de coquillages.");
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

#[command]
fn etat(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut arg_ok = true;
    let id = args.single::<usize>().unwrap_or_else(|_| {
        arg_ok = false; 0
    });
    if !arg_ok {
        send_message(message, &context.http, "Usage : !etat id");
        return Ok(())
    }

    let data = context.data.read();
    let state = match data.get::<BetStateData>().unwrap().get(&id) {
        Some(state) => state,
        None => {
            send_message(message, &context.http, "Je ne connais pas cet id !");
            return Ok(())
        },
    };
    let mut reply = MessageBuilder::new();
    match state {
        BetState::Betting => reply.push("Les paris sont ouverts !\n"),
        BetState::WaitingResult => reply.push("Les paris sont fermés !\n"),
        BetState::NotBetting => {
            reply.push("Les paris n'ont pas commencés.");
            let reply = reply.build();
            send_message(message, &context.http, &reply);
            return Ok(())
        }
    };
    let game = match data.get::<GameData>().unwrap()[id].as_ref() {
        Some(g) => g,
        None => return Ok(()),
    };
    let conn = connect_db();
    let game = get_game(game.0.clone(), game.1.clone(), &conn).unwrap();
    let reply = reply.push(format!("Total pour {} : {}", game.black, game.black_bet))
                    .push(format!("\nTotal pour {} : {}", game.white, game.white_bet))
                    .build();
    send_message(message, &context.http, &reply);

    Ok(())
}
