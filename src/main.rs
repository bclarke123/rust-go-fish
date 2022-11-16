use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum CardType {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Card<'a> {
    suit: &'a Suit,
    card_type: &'a CardType,
}

impl<'a> Card<'a> {
    fn new(suit: &'a Suit, card_type: &'a CardType) -> Card<'a> {
        Card::<'a> { suit, card_type }
    }
}

impl<'a> Ord for Card<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.suit > other.suit {
            return Ordering::Greater;
        }

        if self.suit < other.suit {
            return Ordering::Less;
        }

        if self.card_type > other.card_type {
            return Ordering::Greater;
        }

        if self.card_type < other.card_type {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

#[derive(Debug)]
struct Deck<'a> {
    cards: Vec<Card<'a>>,
}

impl<'a> Deck<'a> {
    fn new() -> Deck<'a> {
        let suits = vec![&Suit::Spade, &Suit::Heart, &Suit::Club, &Suit::Diamond];

        let values = (0..13)
            .map(|x| match x {
                0 => &CardType::Ace,
                1 => &CardType::Jack,
                2 => &CardType::Number(2),
                3 => &CardType::Number(3),
                4 => &CardType::Number(4),
                5 => &CardType::Number(5),
                6 => &CardType::Number(6),
                7 => &CardType::Number(7),
                8 => &CardType::Number(8),
                9 => &CardType::Number(9),
                10 => &CardType::Number(10),
                11 => &CardType::Queen,
                12 => &CardType::King,
                _ => panic!(),
            })
            .collect::<Vec<&CardType>>();

        let cards = (0..52)
            .map(|value| {
                let suit = suits[value % 4];
                let card_type = values[value % 13];

                Card::new(suit, card_type)
            })
            .collect::<Vec<Card>>();

        let mut cards = cards;
        cards.sort();
        let cards = cards;

        Deck::<'a> { cards }
    }

    fn shuffle(&mut self) {
        let cards = &mut self.cards;
        let mut rng = rand::thread_rng();

        let len = cards.len() - 1;
        for i in 0..len {
            let index = len - i;
            let random = rng.gen_range(0..index);

            cards.swap(index, random);
        }
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    println!("{:?}", deck);
}
