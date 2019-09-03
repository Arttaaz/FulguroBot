DROP TABLE game;

CREATE TABLE game (
    black     TEXT NOT NULL,
    white     TEXT NOT NULL,
    black_bet INT NOT NULL,
    white_bet INT NOT NULL,
    state     INT NOT NULL,

    PRIMARY KEY (black, white)
);
