

const BLACKJACK      : u8 = 21;

#[derive(Debug)]
pub struct Hand {
    cards : Vec<char>, 
    count_a : u8,
    count_b : u8,
}

impl Hand {
    pub fn new(cards : Vec<char>) -> Hand {
        Hand {
            cards : cards,
            count_a : 0,
            count_b : 0,
        }
    }
    
    pub fn add_card(&mut self, card : char) -> bool {
        self.cards.push(card);
        self.update_count(card);
        return self.is_busted() == false;
    }

    fn update_count(&mut self, card : char) {
        match card {
            '2' => {self.count_a += 2; self.count_b += 2;},
            '3' => {self.count_a += 3; self.count_b += 3;},
            '4' => {self.count_a += 4; self.count_b += 4;},
            '5' => {self.count_a += 5; self.count_b += 5;},
            '6' => {self.count_a += 6; self.count_b += 6;},
            '7' => {self.count_a += 7; self.count_b += 7;},
            '8' => {self.count_a += 8; self.count_b += 8;},
            '9' => {self.count_a += 9; self.count_b += 9;},
            'T' | 'J' | 'Q' | 'K' => {self.count_a += 10; self.count_b += 10;},
            'A' => {self.count_a += 1; self.count_b += 11;},
            _=> panic!("Invalid card provided"),
        }
    }

    pub fn is_busted(&mut self) -> bool {
        if self.count_a > BLACKJACK && self.count_b > BLACKJACK {
            return true;
        }
        return false;
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        self.count_a = 0;
        self.count_b = 0;
    }

    pub fn display_hand(&mut self) {
        let cards: &Vec<char> = &self.cards;
        println!("{}", construct_top_row(cards));
        println!("{}", construct_upper_value_row(cards));
        println!("{}", construct_upper_suit_row(cards));
        println!("{}", construct_lower_suit_row(cards));
        println!("{}", construct_lower_value_row(cards));
        println!("{}", construct_bottom_row(cards));
    }

    pub fn display_hidden_hand(&mut self) {
        let cards: &Vec<char> = &self.cards[1..].to_vec();


        let hidden_top:    &str= ".------.";
        let hidden_middle: &str= "|      |";
        let hidden_bottom: &str= "`------'";

        println!("{} {}", hidden_top , construct_top_row(cards));
        println!("{} {}", hidden_middle , construct_upper_value_row(cards));
        println!("{} {}", hidden_middle , construct_upper_suit_row(cards));
        println!("{} {}", hidden_middle , construct_lower_suit_row(cards));
        println!("{} {}", hidden_middle , construct_lower_value_row(cards));
        println!("{} {}", hidden_bottom ,construct_bottom_row(cards));
    }
}


fn construct_top_row(cards: &Vec<char>) -> String {
    let mut row_graphic: String= String::new();
    for _card in cards {
        row_graphic += ".------.";
        row_graphic += " ";
    }
    return row_graphic;    
}

fn construct_upper_value_row(cards: &Vec<char>) -> String {
    let mut row_graphic: String= String::new();
    for card in cards {
        row_graphic += "|";
        row_graphic.push(*card);
        row_graphic += ".--. |";
        row_graphic += " ";
    }
    return row_graphic;    
}

fn construct_upper_diamond() -> String { return "| :/\\: |".to_string(); }
fn construct_lower_diamond() -> String { return "| :\\/: |".to_string(); }

fn construct_upper_club() -> String { return "| :(): |".to_string(); }
fn construct_lower_club() -> String { return "| ()() |".to_string(); }

fn construct_upper_heart() -> String { return "| (\\/) |".to_string(); }
fn construct_lower_heart() -> String { return "| :\\/: |".to_string(); }

fn construct_upper_spade() -> String { return "| :/\\: |".to_string(); }
fn construct_lower_spade() -> String { return "| (__) |".to_string(); }

fn construct_upper_suit_row(cards: &Vec<char>) -> String {
    let mut row_graphic: String= String::new();

    let num_of_cards = cards.len();
    for card in 0..num_of_cards {
        match card % 4 {
            0 => row_graphic += &construct_upper_diamond(),
            1 => row_graphic += &construct_upper_club(),
            2 => row_graphic += &construct_upper_heart(),
            3 => row_graphic += &construct_upper_spade(),
            _=> row_graphic += &construct_upper_diamond(),
        } 
        row_graphic += " ";
    }
    return row_graphic;    
}

fn construct_lower_suit_row(cards: &Vec<char>) -> String {
    let mut row_graphic: String= String::new();

    let num_of_cards = cards.len();
    for card in 0..num_of_cards {
        match card % 4 {
            0 => row_graphic += &construct_lower_diamond(),
            1 => row_graphic += &construct_lower_club(),
            2 => row_graphic += &construct_lower_heart(),
            3 => row_graphic += &construct_lower_spade(),
            _=> row_graphic += &construct_lower_diamond(),
        } 
        row_graphic += " ";
    }
    return row_graphic;    
}

fn construct_lower_value_row(cards: &Vec<char>) -> String {
    let mut row_graphic: String= String::new();
    for card in cards {
        row_graphic += "| '--'";
        row_graphic.push(*card);
        row_graphic += "|";
        row_graphic += " ";
    }
    return row_graphic;    
}

fn construct_bottom_row(cards: &Vec<char>) -> String {
    let mut row_graphic: String= String::new();
    for _card in cards {
        row_graphic += "`------'";
        row_graphic += " ";
    }
    return row_graphic;    
}


#[cfg(test)]
mod hand_tests {
    use super::*;

    #[test]
    fn hand_add_card() {
        let mut hand = Hand::new(Vec::new());
        assert_eq!(hand.cards.len(), 0);
        assert_eq!(hand.count_a, 0);
        assert_eq!(hand.count_b, 0);
        assert_eq!(hand.is_busted(), false);

        assert_eq!(hand.add_card('2'), true);
        assert_eq!(hand.cards.len(), 1);
        assert_eq!(hand.count_a, 2);
        assert_eq!(hand.count_b, 2);

        assert_eq!(hand.add_card('T'), true);
        assert_eq!(hand.cards.len(), 2);
        assert_eq!(hand.count_a, 12);
        assert_eq!(hand.count_b, 12);

        assert_eq!(hand.add_card('A'), true);
        assert_eq!(hand.cards.len(), 3);
        assert_eq!(hand.count_a, 13);
        assert_eq!(hand.count_b, 23);

        assert_eq!(hand.add_card('K'), false);
        assert_eq!(hand.cards.len(), 4);
        assert_eq!(hand.count_a, 23);
        assert_eq!(hand.count_b, 33);
    }
}