#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::insert_into;
use std::env;
use dotenv::dotenv;
use diesel::prelude::*;

mod models;
mod schema;

use crate::models::*;
use crate::schema::*;

const NB_BASE_COQ : i32 = 1000;
const NB_BASE_BOOST: i32 = 5;


pub fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url).expect("Error connecting to database")
}

//////////////////////////////
// USER
//////////////////////////////

pub fn create_user(id: String, name: String, conn: &SqliteConnection) {
    let user = Users {
        id,
        name,
        nb_coq: NB_BASE_COQ,
        nb_boost: NB_BASE_BOOST,
    };
    insert_into(users::dsl::users).values(user).execute(conn).expect("Failed to add user");
}

pub fn user_exists(id: String, conn: &SqliteConnection) -> bool {

    let result = users::dsl::users.filter(users::dsl::id.eq(id)).first::<Users>(conn);

    if let Err(_notfound) = result {
        false
    } else {
        true
    }
}

pub fn get_users_bet_color(black: String, white: String, color: String, conn: &SqliteConnection) -> Option<Vec<Users>> {
    match diesel::dsl::sql_query(
        format!("SELECT * FROM users WHERE id IN (SELECT user_id FROM bets WHERE black = \"{}\" AND white = \"{}\" AND color = \'{}\')", black, white, color))
        .load(conn) {

        Ok(users) => Some(users),
        _ => None,
    }
}

pub fn add_coq_to_user(id: String, nb_coq: i32, conn: &SqliteConnection) {
    let coq = get_coq_of_user(id.clone(), conn);
    set_coq_to_user(id, coq + nb_coq, conn);
}

fn set_coq_to_user(id: String, nb_coq: i32, conn: &SqliteConnection) {
    diesel::update(users::dsl::users.find(id)).set(users::dsl::nb_coq.eq(nb_coq)).execute(conn).expect("Failed to update nb_coq");
}

pub fn get_coq_of_user(id: String, conn: &SqliteConnection) -> i32 {
    match users::dsl::users.select(users::dsl::nb_coq).filter(users::dsl::id.eq(id)).first::<i32>(conn) {
        Ok(nb_coq) => nb_coq,
        Err(_) => -1,
    }
}

pub fn get_boost_user(id: String, conn: &SqliteConnection) -> i32 {
    match users::dsl::users.select(users::dsl::nb_boost).filter(users::dsl::id.eq(id)).first::<i32>(conn) {
        Ok(nb_boost) => nb_boost,
        Err(_) => -1,
    }
}

pub fn update_boost_user(id: String, modifier: i32, conn: &SqliteConnection) -> Option<i32> {
    let nb_boost = get_boost_user(id.clone(), conn);
    if nb_boost != -1 {
        if nb_boost + modifier >= 0 {
            diesel::update(users::dsl::users.find(id)).set(users::dsl::nb_boost.eq(nb_boost+modifier)).execute(conn)
                .expect("Could not update nb_boost");
            return Some(nb_boost+modifier);
        } else {
            return None;
        }
    }
    None
}

pub fn trade_coq(id_src: String, id_dst: String, nb_coq: i32, conn: &SqliteConnection) -> Result<(),diesel::result::Error> {
    conn.transaction::<_,diesel::result::Error,_>(|| {
        let coq = get_coq_of_user(id_src.clone(), conn);
        if coq - nb_coq >= 0 {
            add_coq_to_user(id_src, -nb_coq, conn);
            add_coq_to_user(id_dst, nb_coq, conn);
            Ok(())
        } else {
            Err(diesel::result::Error::RollbackTransaction)
        }
    })?;

    Ok(())
}

///////////////////////////////////
// BETS
///////////////////////////////////

fn add_bet(user_id: String, black: String, white: String, bet: i32, color: String, conn: &SqliteConnection) {
    let bet = Bets {
        user_id,
        black,
        white,
        bet,
        color,
    };
    insert_into(bets::dsl::bets).values(bet).execute(conn).expect("failed to insert bet");
}

pub fn create_bet(user_id: String, black: String, white: String, bet: i32, color: String, conn: &SqliteConnection) {
    conn.transaction::<_,diesel::result::Error,_>(|| {
        if let Some(old_bet) = get_bet(user_id.clone(), black.clone(), white.clone(), conn) {
            add_coq_to_user(user_id.clone(), old_bet.bet, conn);
            let mut game = get_game(old_bet.black.clone(), old_bet.white.clone(), conn).unwrap();
            match old_bet.color.as_str() {
                "noir" => {
                    game.black_bet -= old_bet.bet;
                    update_game_bet(old_bet.black.clone(), old_bet.white.clone(), old_bet.color.clone(), game.black_bet, conn);
                },
                "blanc" => {
                    game.white_bet -= old_bet.bet;
                    update_game_bet(old_bet.black.clone(), old_bet.white.clone(), old_bet.color.clone(), game.white_bet, conn);

                },
                _ => (),
            }
            remove_bet(old_bet, conn);
        }
        add_coq_to_user(user_id.clone(), -bet, conn);
        let game = get_game(black.clone(), white.clone(), conn).unwrap();
        match color.as_str() {
            "noir" => {
                update_game_bet(black.clone(), white.clone(), color.clone(), game.black_bet + bet, conn);
            },
            "blanc" => {
                update_game_bet(black.clone(), white.clone(), color.clone(), game.white_bet + bet, conn);
            },
            _ => (),
        }
        add_bet(user_id, black, white, bet, color, conn);

        Ok(())
    }).expect("Could not create bet");
}

pub fn get_bet(user_id: String, black: String, white: String, conn: &SqliteConnection) -> Option<Bets>{
    match bets::dsl::bets
        .filter(bets::dsl::user_id.eq(user_id))
        .filter(bets::dsl::black.eq(black))
        .filter(bets::dsl::white.eq(white))
        .first::<Bets>(conn) {

        Ok(bet) => Some(bet),
        _ => None,
    }
}

/// bet must have same primary key as previous bet (user_id, black and white attributes)
pub fn update_bet(bet: Bets, conn: &SqliteConnection) {
    diesel::update(bets::dsl::bets.find((bet.user_id.clone(), bet.black.clone(), bet.white.clone())))
        .set(bet)
        .execute(conn)
        .expect("Could not update bet");
}

pub fn remove_bets_of_game(black: String, white: String, conn: &SqliteConnection) {
    diesel::delete(bets::dsl::bets
        .filter(bets::dsl::black.eq(black.clone()))
        .filter(bets::dsl::white.eq(white.clone())))
        .execute(conn).unwrap_or_else(|_| panic!("Could not delete bets of game: {} vs {}", black, white));
}

fn remove_bet(bet: Bets, conn: &SqliteConnection) {
    diesel::delete(bets::dsl::bets.find((bet.user_id, bet.black, bet.white))).execute(conn)
        .expect("Could not remove bet");
}

///////////////////////////////
// GAME
///////////////////////////////

pub fn create_game(black: String, white: String, conn: &SqliteConnection) {
    let game = Game {
        black,
        white,
        black_bet: 0,
        white_bet: 0,
        state: 0,
    };

    insert_into(game::dsl::game).values(game).execute(conn).expect("Could not create game");
}

pub fn get_game(black: String, white: String, conn: &SqliteConnection) -> Option<Game> {
    match game::dsl::game.filter(game::dsl::black.eq(black)).filter(game::dsl::white.eq(white)).first::<Game>(conn) {
        Ok(game) => Some(game),
        _ => None,
    }
}

pub fn get_games(conn: &SqliteConnection) -> Vec<Game> {
    game::dsl::game.load::<Game>(conn).unwrap()
}

pub fn update_game_bet(black: String, white: String, color: String, new_total: i32, conn: &SqliteConnection) {
    match color.as_str() {
        "noir" => { diesel::update(game::dsl::game).set(game::dsl::black_bet.eq(new_total))
                        .filter(game::dsl::black.eq(black))
                        .filter(game::dsl::white.eq(white))
                        .execute(conn)
                        .expect("Could not update game");
                    },
        "blanc" => { diesel::update(game::dsl::game).set(game::dsl::white_bet.eq(new_total))
                        .filter(game::dsl::black.eq(black))
                        .filter(game::dsl::white.eq(white))
                        .execute(conn)
                        .expect("Could not update game");
                    },
        _ => ()
    }
}

pub fn update_game_state(black: String, white: String, state: i32, conn: &SqliteConnection) {
    diesel::update(game::dsl::game).set(game::dsl::state.eq(state))
            .filter(game::dsl::black.eq(black))
            .filter(game::dsl::white.eq(white))
            .execute(conn)
            .expect("Could not update game state");
}

pub fn delete_game(black: String, white: String, conn: &SqliteConnection) {
    diesel::delete(game::dsl::game).filter(game::dsl::black.eq(black)).filter(game::dsl::white.eq(white)).execute(conn)
        .expect("Could not delete game");
}

#[cfg(test)]
fn reset_database(conn: &SqliteConnection) {
    diesel::delete(bets::dsl::bets).execute(conn);
    diesel::delete(game::dsl::game).execute(conn);
    diesel::delete(users::dsl::users).execute(conn);
}

#[test]
fn test_get_users_bet_color() {
    let conn = connect_db();
    reset_database(&conn);

    create_user(0, "Romain Fecher".to_string(), &conn);
    add_bet(0, "gne".to_string(), "gne".to_string(), 42, "blanc".to_string(), &conn);

    let expected_users = vec![Users {
        id: 0,
        name: "Romain Fecher".to_string(),
        nb_coq: 1000,
    }];

    assert_eq!(get_users_bet_color("gne".to_string(), "gne".to_string(), "blanc".to_string(), &conn).unwrap(), expected_users);
    reset_database(&conn);
}
