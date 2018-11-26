const DAO = require('./DAO');

class Game {
  /**
  *   create a new game object
  *
  *   @param user1 string : black player's name
  *   @param user2 string : white player's name
  **/
  constructor(user1, user2) {
    this.black = user1;
    this.white = user2;
    this.blackBet = 0;
    this.whiteBet = 0;
    this.bets = {
      black = [],
      white []
    };
  }

  public function get_totat_bet() {
    return this.blackBet + this.whiteBet;
  }

  public function add_bet(color, user, bet) {
    switch (color) {
      case 'black':
        this.blackBet += bet;
        this.bets.black[] = {
          user = user,
          bet = bet
        }
        break;
      case 'white':
        this.whiteBet += bet;
        this.bets.white[] = {
          user = user,
          bet = bet
        }
      default:
        break;
    }
    DAO.add_bet_to_game(this, user, bet, color);
  }

}
