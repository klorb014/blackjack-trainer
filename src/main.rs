

use crate::game::hand::Hand;
use crate::game::shoe::Shoe;

mod game;

use core::num;
use std::cmp;
use std::thread::{sleep, self};
use std::time::Duration;



#[derive(Debug)]
pub struct Game {
    num_of_players : u8,
    shoe         : Shoe,
    dealer_hand  : Hand,
    player_hands : Vec<Hand>,
}

impl Game {
    pub fn new(num_of_players : u8, num_of_decks : u8, penetration_percentage : u8) -> Game {
        let mut shoe: Shoe = Shoe::new(num_of_decks, penetration_percentage, Vec::new());
        shoe.init();

        let mut dealer_hand: Hand = Hand::new(Vec::new());
        let mut player_hands: Vec<Hand> = Vec::new();
        for player in 0..num_of_players {
            player_hands.push(Hand::new(Vec::new()));
        }

        Game {
            num_of_players : num_of_players,
            shoe           : shoe,
            dealer_hand    : dealer_hand,
            player_hands   : player_hands,
        }
    }

    pub fn play(&mut self) {

        // TODO: Add betting mechanism

        self.shoe.deal(&mut self.player_hands, &mut self.dealer_hand);

        display_break();
        display_dealer();
        self.dealer_hand.display_hidden_hand();
        display_player();
        self.player_hands[0].display_hand();


        let player_outcome: PlayerOutcome = self.players_turn();

        let dealer_outcome: PlayerOutcome = self.dealer_turn();

        if player_outcome == PlayerOutcome::Stand && dealer_outcome == PlayerOutcome::Stand{
            
            if cmp::max(self.dealer_hand.count_a, self.dealer_hand.count_b) >= cmp::max(self.player_hands[0].count_a, self.player_hands[0].count_b) {
                display_lose();
            }
            else {
                display_win();
            }

            println!("dealer_max({},{}) = {}, player_max({}, {}) = {}", self.dealer_hand.count_a,
                                                                        self.dealer_hand.count_b,
                                                                        cmp::max(self.dealer_hand.count_a, self.dealer_hand.count_b),
                                                                        self.player_hands[0].count_a,
                                                                        self.player_hands[0].count_b,
                                                                        cmp::max(self.player_hands[0].count_a, self.player_hands[0].count_b));
            
        }
        else if player_outcome == PlayerOutcome::Stand && dealer_outcome == PlayerOutcome::Bust {
            display_win();
        }
        else if player_outcome == PlayerOutcome::Bust && dealer_outcome == PlayerOutcome::Stand {
            display_lose();
        }
        else if player_outcome == PlayerOutcome::Bust && dealer_outcome == PlayerOutcome::Bust {
            display_lose();
        }

    }

    fn players_turn(&mut self) -> PlayerOutcome {

        let player_hand: &mut Hand = &mut self.player_hands[0];

        let mut outcome: PlayerOutcome = PlayerOutcome::InProgress;
        
        while outcome == PlayerOutcome::InProgress {

            let action: UserAction = get_user_action();

            let basic_strat_action: UserAction = hard_total_basic_strategy(player_hand, &mut self.dealer_hand);
            if action != basic_strat_action {
                println!("WRONG! Correct Choice {:?}", basic_strat_action);
            }
            else {
                println!("CORRECT! Choice {:?}", basic_strat_action);
            }

            match action {
                UserAction::Hit     => outcome = if self.shoe.hit(player_hand) { PlayerOutcome::InProgress } else { PlayerOutcome::Bust },
                UserAction::Stand   => outcome = PlayerOutcome::Stand,
                UserAction::Unknown => println!("UserAction::Unknown"),
                UserAction::DoubleDown => { self.shoe.hit(&mut self.dealer_hand); outcome = PlayerOutcome::Stand },
            }

            display_break();
            display_dealer();
            self.dealer_hand.display_hidden_hand();
            display_player();
            player_hand.display_hand();
            if player_hand.is_busted() {
                display_bust();
            }
        }

        return outcome;
    }

    fn dealer_turn(&mut self) -> PlayerOutcome {

        let player_hand: &mut Hand = &mut self.player_hands[0];
        let mut outcome: PlayerOutcome = PlayerOutcome::InProgress;
        
        while outcome == PlayerOutcome::InProgress {

            let mut action: UserAction = UserAction::Hit;

            if self.dealer_hand.count_a >= 17 || self.dealer_hand.count_b >= 17 {
                action = UserAction::Stand;
            }
            
            match action {
                UserAction::Hit     => outcome = if self.shoe.hit(&mut self.dealer_hand) { PlayerOutcome::InProgress } else { PlayerOutcome::Bust },
                UserAction::Stand   => outcome = PlayerOutcome::Stand,
                UserAction::Unknown => println!("UserAction::Unknown"),
                UserAction::DoubleDown => println!("UserAction::DoubleDown"),
            }

            display_break();
            display_dealer();
            self.dealer_hand.display_hand();
            if self.dealer_hand.is_busted() {
                display_bust();
            }
            display_player();
            player_hand.display_hand();
            if player_hand.is_busted() {
                display_bust();
            }

            thread::sleep(Duration::from_secs(1));
        }

        return outcome;
    }

}


#[derive(PartialEq)]
enum PlayerOutcome {
    Bust,
    Stand,
    // BlackJack,
    InProgress,
}

#[derive(Debug, PartialEq)]
enum UserAction {
    Hit,
    Stand,
    DoubleDown,
    Unknown,
}

fn user_input_to_action(input: &str) -> UserAction {
    // let lowercase_input: String = input.to_lowercase();
    match input {
        "h" => return UserAction::Hit,
        "s" => return UserAction::Stand,
        "d" => return UserAction::DoubleDown,
        _=>    return UserAction::Unknown,
    }
}

fn get_user_action() -> UserAction {

    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("(H)it, (S)tand, or (D)ouble Down: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}",s);
    return user_input_to_action(&s);
}



fn display_bust() {
    let busted_str: &str = " ___ _   _ ___ _____ _ \n\
                            | _ ) | | / __|_   _| |\n\
                            | _ \\ |_| \\__ \\ | | |_|\n\
                            |___/\\___/|___/ |_| (_)";
    println!("{}", busted_str);
}

fn display_win() {
    let win_str: &str = " __      _____ _  _ _ \n \
                          \\ \\    / /_ _| \\| | |\n  \
                           \\ \\/\\/ / | || .` |_|\n   \
                            \\_/\\_/ |___|_|\\_(_)";
    println!("{}", win_str);
}

fn display_lose() {
    let lose_str: &str = " _    ___  ___ ___ _ \n\
                          | |  / _ \\/ __| __| |\n\
                          | |_| (_) \\__ \\ _||_|\n\
                          |____\\___/|___/___(_)";
    println!("{}", lose_str);
}

fn display_dealer() {
    let dealer_str: &str = "     ___           __       \n    \
                               / _ \\___ ___ _/ /__ ____\n   \
                              / // / -_) _ `/ / -_) __/\n  \
                             /____/\\__/\\_,_/_/\\__/_/   ";
    println!("{}", dealer_str);
}

fn display_player() {
    let player_str: &str = "   ___  __                 \n  \
                              / _ \\/ /__ ___ _____ ____\n \
                             / ___/ / _ `/ // / -_) __/\n\
                            /_/  /_/\\_,_/\\_, /\\__/_/   \n            \
                                          /___/          ";
    println!("{}", player_str);
}

fn display_break() {
    let break_str: &str = "                                                                                                                                                                 \n \
                            ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______   ______ \n\
                           /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/ \n\
                           /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/  /_____/ \n\
                                                                                                                                                                                            \n\
                                                                                                                                                                                            ";
    println!("{}", break_str);
}


fn hard_total_basic_strategy(player_hand : &mut Hand, dealer_hand : &mut Hand) -> UserAction {
    // A hard total is any hand that does not start with an ace in it, 
    // or it has been dealt an ace that can only be counted as 1 instead of 11.
    
    let dealer_upcard: char = dealer_hand.cards[1];
    let player_total: u8 = player_hand.count_a;

    // 17 and up always stands
    if player_total >= 17 { return UserAction::Stand; }
    
    // 16 stands against dealer 2 through 6, otherwise hit.
    if player_total == 16 && dealer_upcard == '2' { return UserAction::Stand; }
    if player_total == 16 && dealer_upcard == '3' { return UserAction::Stand; }
    if player_total == 16 && dealer_upcard == '4' { return UserAction::Stand; }
    if player_total == 16 && dealer_upcard == '5' { return UserAction::Stand; }
    if player_total == 16 && dealer_upcard == '6' { return UserAction::Stand; }
    if player_total == 16  { return UserAction::Hit; }

    // 15 stands against dealer 2 through 6, otherwise hit.
    if player_total == 15 && dealer_upcard == '2' { return UserAction::Stand; }
    if player_total == 15 && dealer_upcard == '3' { return UserAction::Stand; }
    if player_total == 15 && dealer_upcard == '4' { return UserAction::Stand; }
    if player_total == 15 && dealer_upcard == '5' { return UserAction::Stand; }
    if player_total == 15 && dealer_upcard == '6' { return UserAction::Stand; }
    if player_total == 15 { return UserAction::Hit; }

    // 14 stands against dealer 2 through 6, otherwise hit.
    if player_total == 14 && dealer_upcard == '2' { return UserAction::Stand; }
    if player_total == 14 && dealer_upcard == '3' { return UserAction::Stand; }
    if player_total == 14 && dealer_upcard == '4' { return UserAction::Stand; }
    if player_total == 14 && dealer_upcard == '5' { return UserAction::Stand; }
    if player_total == 14 && dealer_upcard == '6' { return UserAction::Stand; }
    if player_total == 14  { return UserAction::Hit; }

    // 13 stands against dealer 2 through 6, otherwise hit.
    if player_total == 13 && dealer_upcard == '2' { return UserAction::Stand; }
    if player_total == 13 && dealer_upcard == '3' { return UserAction::Stand; }
    if player_total == 13 && dealer_upcard == '4' { return UserAction::Stand; }
    if player_total == 13 && dealer_upcard == '5' { return UserAction::Stand; }
    if player_total == 13 && dealer_upcard == '6' { return UserAction::Stand; }
    if player_total == 13  { return UserAction::Hit; }

    // 12 stands against dealer 4 through 6, otherwise hit.
    if player_total == 12 && dealer_upcard == '4' { return UserAction::Stand; }
    if player_total == 12 && dealer_upcard == '5' { return UserAction::Stand; }
    if player_total == 12 && dealer_upcard == '6' { return UserAction::Stand; }
    if player_total == 12 { return UserAction::Hit; }

    // 11 always doubles.
    if player_total == 11 { return UserAction::DoubleDown; }
    
    // 10 doubles against dealer 2 through 9 otherwise hit.
    if player_total == 10 && dealer_upcard == 'T' { return UserAction::Hit; }
    if player_total == 10 && dealer_upcard == 'J' { return UserAction::Hit; }
    if player_total == 10 && dealer_upcard == 'Q' { return UserAction::Hit; }
    if player_total == 10 && dealer_upcard == 'K' { return UserAction::Hit; }
    if player_total == 10 && dealer_upcard == 'A' { return UserAction::Hit; }
    if player_total == 10 { return UserAction::DoubleDown; }

    // 9 doubles against dealer 3 through 6 otherwise hit.
    if player_total == 9 && dealer_upcard == '3' { return UserAction::DoubleDown; }
    if player_total == 9 && dealer_upcard == '4' { return UserAction::DoubleDown; }
    if player_total == 9 && dealer_upcard == '5' { return UserAction::DoubleDown; }
    if player_total == 9 && dealer_upcard == '6' { return UserAction::DoubleDown; }
    if player_total == 9 { return UserAction::Hit; }

    // 8 always hits.
    if player_total == 8  { return UserAction::Hit; }


    else { return UserAction::Unknown; }
}
                                                                   

fn main() {

    let mut game: Game = Game::new(1, 1, 50);
    game.play();
    

}


