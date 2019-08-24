use std::fmt::Display;
use diesel::Queryable;
use diesel::Insertable;
use crate::schema::*;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name= "bets"]
pub struct Bets {
    pub user_id: String,
    pub black: String,
    pub white: String,
    pub bet: i32,
    pub color: String,
}

impl Display for Bets {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let conn = crate::connect_db();
        let name = crate::get_user(self.user_id.clone(), &conn).unwrap().name;
        write!(f, "{} : {}", name, self.bet)
    }
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "game"]
pub struct Game {
    pub black: String,
    pub white: String,
    pub black_bet: i32,
    pub white_bet: i32,
    pub state: i32,
}

#[derive(Debug, PartialEq, Queryable, QueryableByName, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: String,
    pub name: String,
    pub nb_coq: i32,
    pub nb_recharge: i32,
}
