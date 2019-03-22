CREATE TABLE USERS(
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
);
