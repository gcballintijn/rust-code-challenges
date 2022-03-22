#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        use Card::*;
        
        // Compute minimal hand value and count aces
        let mut min_hand_value = 0;
        let mut ace_count = 0;
        for card in &self.cards {
            let min_card_value = match card {
                Ace=> { ace_count += 1; 1 },
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                Jack => 10,
                Queen => 10,
                King => 10,                            
            };
            min_hand_value += min_card_value;
        }

        let mut result = min_hand_value;
        for _ in 0..ace_count {
            if result >= 21 {
                break;
            }

            result = if (result + 10) <= 21 {
                result + 10
            } else {
                result
            };
        }

        result
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);

    println!("The value of King and Ace is {} and this {} a loosing hand.",
        hand.value(),
        if hand.is_loosing_hand() { "IS" } else { "is not" }
    );

    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Two);

    println!("The value of King, Queen, and Two is {} and this {} a loosing hand.",
        hand.value(),
        if hand.is_loosing_hand() { "IS" } else { "is not" }
    );
}


#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    
    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);
    
    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}

#[test]
fn low_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Three);
    hand.add(Card::Three);
    hand.add(Card::Seven);
    
    assert_eq!(hand.value(), 13);
}

#[test]
fn four_aces() {
    let mut hand = Hand::new();
    hand.add(Card::Ace);
    hand.add(Card::Ace);
    hand.add(Card::Ace);
    hand.add(Card::Ace);
    
    assert_eq!(hand.value(), 14);
}

#[test]
fn is_loosing_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Two);
    
    assert!(hand.is_loosing_hand());
}

#[test]
fn is_not_loosing_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Two);
    
    assert!(!hand.is_loosing_hand());
}
