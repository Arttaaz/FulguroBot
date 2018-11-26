const sqlite = require('sqlite3').verbose();

module.exports = {

  /**
  *   return a connection to the database
  **/
  function connect_db() {
    return new sqlite.Database("./fulguro_base.db", (err) => {
      if(err) {
        return console.error(err.message);
      }
    });  //bot has to close the database himself when he is done using it
  }

  /**
  *   return all users from database
  **/
  function get_users() {
    db = connect_db();
    let str = "SELECT * FROM USERS;";
    rows = db.all(str, (err, rows) => {
      if (err) {
        return console.error(err.message);
      }
      return rows;

    });
    db.close();
    return rows;
  }

  /**
  *   return user from id
  *
  *   @param id string : id of the user to find
  **/
  function get_user(id) {
    db = connect_db();
    let str = "SELECT * FROM USERS WHERE id = \"" + id + "\"";
    rows = db.get(str, (err, row) => {
      if (err) {
        return console.error(err.message);
      }
      return row;
    });
    db.close();
    return row;
  }

  /**
  *   add a new user to the database
  *
  *   @param id string        : snowflake id of new user
  *   @param name string      : name of the new user
  *   @param coquillages int  : number of coquillages for the new user
  **/
  function add_user(id, name, coquillages) {
    let sql = "INSERT INTO USERS (id, name, coquillages) VALUES (\""+ id +"\", \"" + name + "\", " + coquillages + ")"
    db = connect_db();
    db.run(sql, (err) => {
      if (err)
        return console.error(err.message);
    });
    db.close();
  }

  /**
  *   @param black string : black player of the game
  *   @param white string : white player of the game
  **/
  function create_game(black, white) {
    let sql = "INSERT INTO GAME (black, white) VALUES (\""+ black + "\", \"" + white + "\");";
    db = connect_db();
    db.run(sql, (err) => {
      if (err) {
        return console.error(err.message);
      }
    });
    db.close();
  }

  /**
  *   add a bet to a player of a game currently running
  *
  *   @param game object  : the game object
  *   @param user object  : the user object from discord
  *   @param bet  int     : number of coquillages bet
  *   @param color string : the target of the bet (white or black)
  **/
  function add_bet_to_game(game, user, bet, color) {
    let sql = 'INSERT INTO BETS (user_id, black, white, bet, color) VALUES ("' + user.id + '", "'+ game.black +'", "'+
        game.white + '", '+ bet + ', "'+ color +'");';
    db = connect_db();
    db.run(sql, (err) => {
      if (err) {
        return console.error(err.message);
      }
    });
    db.close();
  }

  /**
  *   return all bets from a game
  *
  *   @param game object : the game object
  **/
  function get_bets_for_game(game) {
    let sql = 'SELECT bets.black, bets.white, users.name, bets.bet, bets.color FROM BETS, USERS WHERE black = "'+ bets.black + '" AND white = "'+ bets.white + '" AND USERS.id = user_id'
    db = connect_db();
    rows = db.all(sql, (err, rows) => {
      if (err) {
        return console.error(err.message);
      }
      return rows;

    });
    db.close();
    return rows;
  }
};
