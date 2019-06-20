use serenity::utils::MessageBuilder;
use serenity::model::channel::Message;
use fulgurobot_db::*;
use super::{BetState, BetStateData};


fn bet_on_color(color: &str, message: &Message , nb_coq: i32) {
    let conn = connect_db();
    let id = message.author.id.0 as i32;

    if !user_exists(id, &conn) {
        add_user(id, message.author.name.clone(), &conn);
    }

    // check if user has enough coq
    let coq = get_coq_of_user(id, &conn);
    if coq - nb_coq > 0 {
        create_bet(id, "todo".to_string(), "todo".to_string(), nb_coq, coq - nb_coq, color.to_string(), &conn);
    } else {
        let reply = MessageBuilder::new()
                    .push_bold_safe(message.author.name.clone())
                    .push(", Tu n'as pas assez de coquillages.")
                    .build();
        message.channel_id.say(&reply).expect("Could not send not enough coq reply");
    }
}


// !fulgurobot
command!(fulgurobot(_context, message) {
    let reply = MessageBuilder::new()
            .push(", Commandes pour parier :\n!noir x -> parie x coquillages sur noir\n!blanc x -> parie x coquillages sur blanc\n!coq -> envoie en message privé votre nombre de coquillages")
            .build();

    if let Err(why) = message.channel_id.say(&reply) {
        println!("error sending message: {:?}", why);
    }
});

// !noir bet
command!(noir(context, message, args) {
    let data = context.data.lock();
    match *data.get::<BetStateData>().unwrap() {
        BetState::NotBetting    => { message.channel_id.say("Il n'y a pas de partie en cours")
                                    .expect("Could not send message"); return Ok(());},
        BetState::WaitingResult => { message.channel_id.say("Les paris sont finis !")
                                    .expect("Could not send message"); return Ok(()); },
        _ => ()
    }

    let nb_coq = args.single::<i32>().unwrap();

    bet_on_color("black", message, nb_coq);
});

// !blanc bet
command!(blanc(context, message, args) {
    let data = context.data.lock();
    match *data.get::<BetStateData>().unwrap() {
        BetState::NotBetting    => { message.channel_id.say("Il n'y a pas de partie en cours")
                                    .expect("Could not send message"); return Ok(());},
        BetState::WaitingResult => { message.channel_id.say("Les paris sont finis !")
                                    .expect("Could not send message"); return Ok(()); },
        _ => ()
    }

    let nb_coq = args.single::<i32>().unwrap();

    bet_on_color("white", message, nb_coq);
});
