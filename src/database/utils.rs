use diesel::insert_into;
use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use super::models::*;
use crate::schema::*;


pub fn connect_db() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url).expect("Error connecting to database")
}

pub fn add_user(id: i32, name: String, conn: &SqliteConnection) {
    let user = Users {
        id,
        name,
        nb_coq: 1000
    };
    insert_into(USERS::dsl::USERS).values(user).execute(conn).expect("Failed to add user");
}

pub fn user_exists(id: i32, conn: &SqliteConnection) -> bool {

    let result = USERS::dsl::USERS.filter(USERS::dsl::id.eq(id)).first::<Users>(conn);

    if let Err(_notfound) = result {
        false
    } else {
        true
    }
}

pub fn add_coq_to_user(id: i32, nb_coq: i32, conn: &SqliteConnection) {
    diesel::update(USERS::dsl::USERS.find(id)).set(USERS::dsl::nb_coq.eq(nb_coq)).execute(conn).expect("Failed to update nb_coq");
}

pub fn get_coq_of_user(id: i32, conn: &SqliteConnection) -> i32 {
    match USERS::dsl::USERS.select(USERS::dsl::nb_coq).filter(USERS::dsl::id.eq(id)).first::<i32>(conn) {
        Ok(nb_coq) => nb_coq,
        Err(_) => -1,
    }
}

pub fn add_bet(user_id: i32, black: String, white: String, bet: i32, color: String, conn: &SqliteConnection) {
    let bet = Bets {
        user_id,
        black,
        white,
        bet,
        color,
    };
    insert_into(BETS::dsl::BETS).values(bet).execute(conn).expect("failed to insert bet");
}
