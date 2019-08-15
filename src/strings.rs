use std::collections::HashMap;
use lazy_static::lazy_static;

#[macro_export]
macro_rules! locale {
    ($locale:ident, $key:tt) => {
        $locale.get($key).unwrap()
    };
}

// some are not used but I prepared for the future
lazy_static! {
    pub static ref FRENCH : HashMap<&'static str, &'static str> = {
        let mut strings : HashMap<&'static str, &'static str> = HashMap::new();

        strings.insert("bet_0", "Usage: !");
        strings.insert("bet_1", " game_id nb_coq");
        strings.insert("bet_2", "Il faut parier plus que 0 coquillages !");
        strings.insert("bet_3", "Les paris n'ont pas démarré.");
        strings.insert("bet_4", "Les paris sont finis !");
        strings.insert("bet_5", "Cette partie n'existe pas");
        strings.insert("bet_6", "Tu n'as pas assez de coquillages.");

        strings.insert("fulgurobot_0", "Commandes pour utiliser Fulgurobot");
        strings.insert("fulgurobot_1", "i correspond à l'identifiant de la partie donné par le bot");
        strings.insert("fulgurobot_2", "!noir i x");
        strings.insert("fulgurobot_3", "parie x coquillages sur noir pour la partie i");
        strings.insert("fulgurobot_4", "!blanc i x");
        strings.insert("fulgurobot_5", "parie x coquillages sur blanc pour la partie i");
        strings.insert("fulgurobot_6", "!coq");
        strings.insert("fulgurobot_7", "envoie en message privé votre nombre de coquillages");
        strings.insert("fulgurobot_8", "!nb_recharge");
        strings.insert("fulgurobot_9", "donne votre nombre de recharges restants");
        strings.insert("fulgurobot_10", "!recharge");
        strings.insert("fulgurobot_11", "vous octroie 200 coquillages en cas de besoin ! (5 utilisations par personne)");

        strings.insert("create_game_0", "usage: !create_game noir blanc");
        strings.insert("create_game_1", "La partie de ");
        strings.insert("create_game_2", " a été créée avec l'id : ");

        strings.insert("debut_paris_0", "Usage: !debut_paris game_id");
        strings.insert("debut_paris_1", "Les paris sont déjà en cours !");
        strings.insert("debut_paris_2", "La partie est en attente du résultat");
        strings.insert("debut_paris_3", "Les paris sont ouverts !");
        strings.insert("debut_paris_4", "Mauvais id de partie");

        strings.insert("fin_paris_0", "Usage: !fin_paris game_id");
        strings.insert("fin_paris_1", "Mauvais id de partie");
        strings.insert("fin_paris_2", "Les paris de la partie ");
        strings.insert("fin_paris_3", " sont finis ! \nTotal pour ");
        strings.insert("fin_paris_4", " (noir) : ");
        strings.insert("fin_paris_5", " coquillages\nTotal pour ");
        strings.insert("fin_paris_6", " (blanc) : ");
        strings.insert("fin_paris_7", " coquillages");

        strings.insert("resultat_0", "Usage: !resultat game_id couleur (blanc ou noir)");
        strings.insert("resultat_1", "Usage: !resultat game_id couleur");
        strings.insert("resultat_2", "Mauvais id de partie");
        strings.insert("resultat_3", "Les paris n'ont même pas encore commencé !");
        strings.insert("resultat_4", "Gagnants :");
        strings.insert("resultat_5", " a gagné ");
        strings.insert("resultat_6", " coquillages !\n");
        strings.insert("resultat_7", "Il n'y a aucun gagnants !");

        strings.insert("coq_0", ", vous avez ");
        strings.insert("coq_1", " coquillages.");

        strings.insert("recharge_0", "Tu as gagné 200 coquillages !\n");
        strings.insert("recharge_1", "Il te reste ");
        strings.insert("recharge_2", " recharges.");
        strings.insert("recharge_3", "Tu n'as plus de recharges !");

        strings.insert("nb_recharge_0", ", il te reste : ");
        strings.insert("nb_recharge_1", " recharges !");

        strings.insert("etat_0", "Usage : !etat id");
        strings.insert("etat_1", "Je ne connais pas cet id !");
        strings.insert("etat_2", "Les paris sont ouverts !\n");
        strings.insert("etat_3", "Les paris sont fermés !\n");
        strings.insert("etat_4", "Les paris n'ont pas commencés.");
        strings.insert("etat_5", "Total pour ");
        strings.insert("etat_6", " (noir) : ");
        strings.insert("etat_7", "\nTotal pour ");
        strings.insert("etat_8", " (blanc) : ");
        strings.insert("etat_9", "Paris sur noir");
        strings.insert("etat_10", "Paris sur blanc");

        strings
    };
}

lazy_static! {
    pub static ref ENGLISH : HashMap<&'static str, &'static str> = {
        let mut strings : HashMap<&'static str, &'static str> = HashMap::new();

        strings.insert("bet_0", "Use: !");
        strings.insert("bet_1", " game_id nb_shell");
        strings.insert("bet_2", "You need to bet more than 0 shells!");
        strings.insert("bet_3", "Bets aren't open yet.");
        strings.insert("bet_4", "Bets are closed.");
        strings.insert("bet_5", "This game doesn't exist.");
        strings.insert("bet_6", "You don't have enough shells.");

        strings.insert("fulgurobot_0", "Commands to use Fulgurobot");
        strings.insert("fulgurobot_1", "i correspond to the id of the game given by the bot.");
        strings.insert("fulgurobot_2", "!black i x");
        strings.insert("fulgurobot_3", "bet x shells on black for the game i");
        strings.insert("fulgurobot_4", "!white i x");
        strings.insert("fulgurobot_5", "bet x shells on white for the game i");
        strings.insert("fulgurobot_6", "!shells");
        strings.insert("fulgurobot_7", "sends in private message your number of shells");
        strings.insert("fulgurobot_8", "!nb_refill");
        strings.insert("fulgurobot_9", "give your number of refill left");
        strings.insert("fulgurobot_10", "!refill");
        strings.insert("fulgurobot_11", "gives you 200 shells in case of need! (5 uses per person)");

        strings.insert("create_game_0", "Use: !create_game black white");
        strings.insert("create_game_1", "The game ");
        strings.insert("create_game_2", " has been created with id : ");

        strings.insert("debut_paris_0", "Use: !debut_paris game_id");
        strings.insert("debut_paris_1", "Les paris sont déjà en cours !");
        strings.insert("debut_paris_2", "La partie est en attente du résultat");
        strings.insert("debut_paris_3", "Les paris sont ouverts !");
        strings.insert("debut_paris_4", "Mauvais id de partie");

        strings.insert("fin_paris_0", "Usage: !fin_paris game_id");
        strings.insert("fin_paris_1", "Mauvais id de partie");
        strings.insert("fin_paris_2", "Les paris de la partie ");
        strings.insert("fin_paris_3", " sont finis ! \nTotal pour ");
        strings.insert("fin_paris_4", " (noir) : ");
        strings.insert("fin_paris_5", " coquillages\nTotal pour ");
        strings.insert("fin_paris_6", " (blanc) : ");
        strings.insert("fin_paris_7", " coquillages");

        strings.insert("resultat_0", "Usage: !resultat game_id couleur (blanc ou noir)");
        strings.insert("resultat_1", "Usage: !resultat game_id couleur");
        strings.insert("resultat_2", "Mauvais id de partie");
        strings.insert("resultat_3", "Les paris n'ont même pas encore commencé !");
        strings.insert("resultat_4", "Gagnants :");
        strings.insert("resultat_5", " a gagné ");
        strings.insert("resultat_6", " coquillages !\n");
        strings.insert("resultat_7", "Il n'y a aucun gagnants !");

        strings.insert("coq_0", ", you have ");
        strings.insert("coq_1", " shells.");

        strings.insert("recharge_0", "You won 200 shells!\n");
        strings.insert("recharge_1", "You have ");
        strings.insert("recharge_2", " refill left.");
        strings.insert("recharge_3", "You have no more refills!");

        strings.insert("nb_recharge_0", ", you have: ");
        strings.insert("nb_recharge_1", " refills left!");

        strings.insert("etat_0", "Use : !etat id");
        strings.insert("etat_1", "I don't know this id!");
        strings.insert("etat_2", "Bets are open!\n");
        strings.insert("etat_3", "Bets are closed!\n");
        strings.insert("etat_4", "Bets aren't open yet!");
        strings.insert("etat_5", "Total for ");
        strings.insert("etat_6", " (black) : ");
        strings.insert("etat_7", "\nTotal for ");
        strings.insert("etat_8", " (white) : ");
        strings.insert("etat_9", "Bets on black");
        strings.insert("etat_10", "Bets on white");

        strings
    };
}
