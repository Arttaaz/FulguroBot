use rusqlite::NO_PARAMS;
use rusqlite::Connection;
use dirs::data_dir;

// check is database exists
// if not creates it.
pub fn init_db() {
    let conn = open_db();

    if let Err(why) = conn.query_row("SELECT * FROM GAME", NO_PARAMS, |_| Ok(())) {
        //database needs to be created
        if why != rusqlite::Error::QueryReturnedNoRows {
            conn.execute_batch(
            "CREATE TABLE USERS(
              id      INT PRIMARY KEY,
              name    TEXT,
              nb_coq  INT
            );

            CREATE TABLE GAME (
                user1     TEXT,
                user2     TEXT,
                black_bet INT,
                white_bet INT,

                PRIMARY KEY (user1, user2)
            );

            CREATE TABLE BETS (
              id    INT PRIMARY KEY,
              user_id INT,
              black TEXT,
              white TEXT,
              bet   INT,
              color TEXT,

              FOREIGN KEY (user_id) REFERENCES USERS(id)
            );").expect("Could not create database");
        }
    }
    conn.close().unwrap();
}

pub fn open_db() -> Connection {
    let mut path = data_dir().unwrap();
    path.push("fulguro_bot/");
    path.push("database.db");

    Connection::open(path).expect("Error opening databse")
}
