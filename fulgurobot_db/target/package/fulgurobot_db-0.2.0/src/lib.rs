#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::error::Error;
use diesel::insert_into;
use std::env;
use dotenv::dotenv;
use diesel::prelude::*;

mod models;
mod schema;

use crate::models::*;
use crate::schema::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



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
    insert_into(users::dsl::users).values(user).execute(conn).expect("Failed to add user");
}

pub fn user_exists(id: i32, conn: &SqliteConnection) -> bool {

    let result = users::dsl::users.filter(users::dsl::id.eq(id)).first::<Users>(conn);

    if let Err(_notfound) = result {
        false
    } else {
        true
    }
}

fn set_coq_to_user(id: i32, nb_coq: i32, conn: &SqliteConnection) {
    diesel::update(users::dsl::users.find(id)).set(users::dsl::nb_coq.eq(nb_coq)).execute(conn).expect("Failed to update nb_coq");
}

pub fn get_coq_of_user(id: i32, conn: &SqliteConnection) -> i32 {
    match users::dsl::users.select(users::dsl::nb_coq).filter(users::dsl::id.eq(id)).first::<i32>(conn) {
        Ok(nb_coq) => nb_coq,
        Err(_) => -1,
    }
}

fn add_bet(user_id: i32, black: String, white: String, bet: i32, color: String, conn: &SqliteConnection) {
    let bet = Bets {
        user_id,
        black,
        white,
        bet,
        color,
    };
    insert_into(bets::dsl::bets).values(bet).execute(conn).expect("failed to insert bet");
}

pub fn create_bet(user_id: i32, black: String, white: String, bet: i32, new_coq: i32, color: String, conn: &SqliteConnection) {
    conn.transaction::<_,diesel::result::Error,_>(|| {
        set_coq_to_user(user_id, new_coq, conn);
        add_bet(user_id, black, white, bet, color, conn);

        Ok(())
    }).expect("Could not create bet");
}
