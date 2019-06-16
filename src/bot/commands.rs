use serenity::utils::MessageBuilder;
use crate::dao::{add_user, add_coq_to_user, user_exists};

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
command!(noir(_context, message) {
    let nb_coq : Vec<&str> = message.content.split(' ').collect();
    let nb_coq : i64 = nb_coq[1].parse().unwrap();
    let id = message.author.id.0 as i64;

    if !user_exists(id) {
        add_user(id, message.author.name.clone(), 1000);
    }
    // maybe change coq number only when game ends to prevent loss at crash
    add_coq_to_user(id, -nb_coq);
    // add nb_coq to bets to black for game
});

// !blanc bet
command!(blanc(_context, message) {
    let nb_coq : Vec<&str> = message.content.split(' ').collect();
    let nb_coq : i64 = nb_coq[1].parse().unwrap();
    let id = message.author.id.0 as i64;

    if !user_exists(id) {
        add_user(id, message.author.name.clone(), 1000);
    }
    add_coq_to_user(id, nb_coq);
    // add nb_coq to bets to white for game
});
