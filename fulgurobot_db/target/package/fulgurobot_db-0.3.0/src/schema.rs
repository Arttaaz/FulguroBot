table! {
    bets (user_id, black, white) {
        user_id -> Integer,
        black -> Text,
        white -> Text,
        bet -> Integer,
        color -> Text,
    }
}

table! {
    game (black, white) {
        black -> Text,
        white -> Text,
        black_bet -> Integer,
        white_bet -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        nb_coq -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    bets,
    game,
    users,
);
