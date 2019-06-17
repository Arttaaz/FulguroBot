table! {
    BETS (id) {
        id -> Integer,
        user_id -> Integer,
        black -> Text,
        white -> Text,
        bet -> Integer,
        color -> Text,
    }
}

table! {
    GAME (black, white) {
        black -> Text,
        white -> Text,
        black_bet -> Integer,
        white_bet -> Integer,
    }
}

table! {
    USERS (id) {
        id -> Integer,
        name -> Text,
        nb_coq -> Integer,
    }
}

joinable!(BETS -> USERS (user_id));

allow_tables_to_appear_in_same_query!(
    BETS,
    GAME,
    USERS,
);
