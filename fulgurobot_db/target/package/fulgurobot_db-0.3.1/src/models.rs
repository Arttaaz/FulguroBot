use diesel::Queryable;
use diesel::Insertable;
use crate::schema::*;

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name= "bets"]
pub struct Bets {
    pub user_id: i32,
    pub black: String,
    pub white: String,
    pub bet: i32,
    pub color: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "game"]
pub struct Game {
    pub black: String,
    pub white: String,
    pub black_bet: i32,
    pub white_bet: i32,
}

#[derive(Debug, PartialEq, Queryable, QueryableByName, Insertable)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub nb_coq: i32,
}
