use diesel::Queryable;
use diesel::Insertable;
use crate::schema::*;

#[derive(Queryable, Insertable)]
#[table_name= "BETS"]
pub struct Bets {
    pub id: i32,
    pub user_id: i32,
    pub black: String,
    pub white: String,
    pub bet: i32,
    pub color: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "GAME"]
pub struct Game {
    pub black: String,
    pub white: String,
    pub black_bet: i32,
    pub white_bet: i32,
}

#[derive(Queryable,Insertable)]
#[table_name = "USERS"]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub nb_coq: i32,
}
