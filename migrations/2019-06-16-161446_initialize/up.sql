CREATE TABLE USERS(
  id      INT NOT NULL PRIMARY KEY,
  name    TEXT NOT NULL,
  nb_coq  INT NOT NULL
);

CREATE TABLE GAME (
    black     TEXT NOT NULL,
    white     TEXT NOT NULL,
    black_bet INT NOT NULL,
    white_bet INT NOT NULL,

    PRIMARY KEY (black, white)
);

CREATE TABLE BETS (
  user_id INT NOT NULL,
  black TEXT NOT NULL,
  white TEXT NOT NULL,
  bet   INT NOT NULL,
  color TEXT NOT NULL,

  PRIMARY KEY (user_id, black, white),
  FOREIGN KEY (user_id) REFERENCES USERS(id)
);
