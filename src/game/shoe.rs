use crate::game::hand::Hand;

use rand::thread_rng;
use rand::seq::SliceRandom;

const CARDS_PER_DECK : u8 = 52;
const CARDS_PER_HAND : u8 = 2;

#[derive(Debug)]
pub struct Shoe {
    num_of_decks: u8,
    num_of_cards: u8,
    penetration_percentage: u8,
    penetration_depth: u8,
    cards: Vec<char>,
}

impl Shoe {
    pub fn new(num_of_decks: u8, penetration_percentage: u8, cards : Vec<char>) -> Shoe {
        let num_of_cards: u8 = num_of_decks * CARDS_PER_DECK;
        let percentage: f32 = penetration_percentage as f32 / 100.0;
        let penetration_depth : f32 = num_of_cards as f32 * percentage;

        Shoe {
            num_of_decks,
            num_of_cards : num_of_cards,
            penetration_percentage,
            penetration_depth : penetration_depth as u8,
            cards,
        }
    }

    pub fn init(&mut self) {
        if self.num_of_decks == 0 ||
            self.penetration_percentage == 0 ||
            self.cards.len() != 0 {
            panic!();
        }
        
        for _deck in 0..self.num_of_decks {
            self.add_deck();
        }

        self.shuffle();
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        self.init();
    }

    pub fn add_deck(&mut self) {
        self.add_suit();
        self.add_suit();
        self.add_suit();
        self.add_suit();
    }

    pub fn add_suit(&mut self) {
        self.cards.push('2');
        self.cards.push('3');
        self.cards.push('4');
        self.cards.push('5');
        self.cards.push('6');
        self.cards.push('7');
        self.cards.push('8');
        self.cards.push('9');
        self.cards.push('T');
        self.cards.push('J');
        self.cards.push('Q');
        self.cards.push('K');
        self.cards.push('A');
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn deal_one(&mut self) -> char {
        if self.cards.len() == 0 {
            panic!("insufficient number of cards left");
        }
        return self.cards.pop().unwrap();
    }

    fn check_penetration_depth(&mut self, num_of_cards_to_deal : usize) -> bool {

        if  num_of_cards_to_deal > self.penetration_depth.into() {
            println!("error: number of hands is greater than shoe size");
            return false;
        }
        let cards_left_in_shoe :usize = self.cards.len();
        let total_num_of_cards : usize = self.num_of_cards.into();

        if total_num_of_cards < cards_left_in_shoe {
            panic!("total_num_of_cards < cards_left_in_shoe");
        }

        let current_penetration_depth : usize = num_of_cards_to_deal as usize + (total_num_of_cards - cards_left_in_shoe);

        println!("current penetration depth: {depth}", depth = current_penetration_depth);

        if current_penetration_depth > self.penetration_depth.into() {
            println!("Shoe penetration depth hit, resetting shoe");
            self.reset();
        }
        else if self.cards.len() < num_of_cards_to_deal.into() {
            println!("insufficient number of cards left, resetting shoe");
            self.reset();
        }

        return true;
    }

    pub fn deal(&mut self, player_hands: &mut Vec<Hand>, dealer_hand : &mut Hand) -> bool {

        let num_of_hands: usize = player_hands.len() + 1;
        let num_of_cards_to_deal : usize = num_of_hands * CARDS_PER_HAND as usize;
        
        if self.check_penetration_depth(num_of_cards_to_deal) == false {
            return false;
        }

        // Deal the dealer's hand
        dealer_hand.reset();
        dealer_hand.add_card(self.deal_one());
        dealer_hand.add_card(self.deal_one());

        // Deal the player hands
        for hand in player_hands {

            hand.reset();
            hand.add_card(self.deal_one());
            hand.add_card(self.deal_one());
        }
        return true;
    }

    pub fn hit(&mut self, hand : &mut Hand) -> bool {

        let num_of_cards_to_deal : usize = 1;
        if self.check_penetration_depth(num_of_cards_to_deal) == false {
            return false;
        }
        
        return hand.add_card(self.deal_one());
    }
}


#[cfg(test)]
mod shoe_tests {
    use super::*;

    #[test]
    fn shoe_constructor() {
        let shoe = Shoe::new( 1, 50, Vec::new());
        assert_eq!(shoe.num_of_decks, 1);
        assert_eq!(shoe.penetration_percentage, 50);
        assert_eq!(shoe.penetration_depth, 26);
        assert_eq!(shoe.cards.len(), 0);
    }

    #[test]
    fn shoe_init() {
        let mut shoe = Shoe::new( 1, 50, Vec::new());
        assert_eq!(shoe.cards.len(), 0);
        shoe.init();
        assert_eq!(shoe.cards.len(), CARDS_PER_DECK.into());

        let mut shoe_2 = Shoe::new( 2, 50, Vec::new());
        assert_eq!(shoe_2.cards.len(), 0);
        shoe_2.init();
        assert_eq!(shoe_2.cards.len(), 104);
    }

    #[test]
    fn shoe_shuffle() {
        let mut unshuffled_shoe = Shoe::new( 1, 50, Vec::new());
        unshuffled_shoe.add_deck();

        let mut shuffled_shoe = Shoe::new( 1, 50, Vec::new());
        shuffled_shoe.init();

        assert_eq!(unshuffled_shoe.cards.len(), shuffled_shoe.cards.len());
        assert_ne!(unshuffled_shoe.cards, shuffled_shoe.cards);
    }

    #[test]
    fn shoe_reset() {
        let mut shoe = Shoe::new( 1, 100, Vec::new());
        shoe.init();

        let mut dealer_hand: Hand = Hand::new(Vec::new());
        let mut hand: Hand = Hand::new(Vec::new());
        let mut player_hands: Vec<Hand> = vec!(hand);

        for _n in 0..10 {
            shoe.deal(&mut player_hands, &mut dealer_hand);
        }
        assert_eq!(shoe.cards.len(), 12);

        shoe.reset();
        assert_eq!(shoe.cards.len(), CARDS_PER_DECK.into());
    }

    #[test]
    fn shoe_deal_one_hand() {
        let mut shoe = Shoe::new( 1, 50, Vec::new());
        shoe.init();

        let mut dealer_hand: Hand = Hand::new(Vec::new());
        let mut hand: Hand = Hand::new(Vec::new());
        let mut player_hands: Vec<Hand> = vec!(hand);

        shoe.deal(&mut player_hands, &mut dealer_hand);
        assert_eq!(shoe.cards.len(), 48);
    }

    #[test]
    fn shoe_deal_until_penetration_depth() {
        let mut shoe = Shoe::new( 1, 50, Vec::new());
        shoe.init();

        let mut dealer_hand: Hand = Hand::new(Vec::new());
        let mut hand: Hand = Hand::new(Vec::new());
        let mut player_hands: Vec<Hand> = vec!(hand);

        for _n in 0..6 {
            assert!(shoe.deal(&mut player_hands, &mut dealer_hand));
        }
        assert_eq!(shoe.cards.len(), 28);
        assert!(shoe.deal(&mut player_hands, &mut dealer_hand));
        assert_eq!(shoe.cards.len(), 48);
    }
}