use rand::Rng;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl Display for Suit {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_str(match *self {
            Suit::Spade => "Spades",
            Suit::Heart => "Hearts",
            Suit::Club => "Clubs",
            Suit::Diamond => "Diamonds",
        })
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CardType {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for CardType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_fmt(format_args!(
            "{}",
            match *self {
                CardType::Ace => "Ace".to_string(),
                CardType::Jack => "Jack".to_string(),
                CardType::Queen => "Queen".to_string(),
                CardType::King => "King".to_string(),
                CardType::Number(x) => format!("{}", x),
            }
        ))
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Card<'a> {
    pub suit: &'a Suit,
    pub card_type: &'a CardType,
}

impl<'a> Card<'a> {
    pub fn new(suit: &'a Suit, card_type: &'a CardType) -> Card<'a> {
        Card::<'a> { suit, card_type }
    }
}

impl<'a> Ord for Card<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.card_type > other.card_type {
            return Ordering::Greater;
        }

        if self.card_type < other.card_type {
            return Ordering::Less;
        }

        if self.suit > other.suit {
            return Ordering::Greater;
        }

        if self.suit < other.suit {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl<'a> Display for Card<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_fmt(format_args!("{} of {}", self.card_type, self.suit))
    }
}

#[derive(Debug)]
pub struct Deck<'a> {
    pub cards: Vec<Card<'a>>,
}

impl<'a> Deck<'a> {
    pub fn new() -> Deck<'a> {
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

        let mut cards = (0..52)
            .map(|value| {
                let suit = suits[value % 4];
                let card_type = values[value % 13];

                Card::new(suit, card_type)
            })
            .collect::<Vec<Card>>();

        cards.sort();

        Deck::<'a> { cards }
    }

    pub fn empty() -> Deck<'a> {
        Deck::<'a> { cards: vec![] }
    }

    pub fn shuffle(&mut self) {
        let cards = &mut self.cards;
        let mut rng = rand::thread_rng();

        let len = cards.len() - 1;
        for i in 0..len {
            let index = len - i;
            let random = rng.gen_range(0..index);

            cards.swap(index, random);
        }
    }

    pub fn sort(&mut self) {
        let cards = &mut self.cards;
        cards.sort();
    }

    pub fn take_card(&mut self) -> Option<Card<'a>> {
        match self.cards.len() {
            0 => None,
            _ => Some(self.cards.remove(0)),
        }
    }

    pub fn give_card(&mut self, card: Card<'a>) {
        self.cards.push(card)
    }

    pub fn give_cards(&mut self, cards: &mut Vec<Card<'a>>) {
        self.cards.append(cards);
    }

    pub fn give_deck(&mut self, deck: &mut Deck<'a>) {
        self.give_cards(&mut deck.cards);
    }

    pub fn pairs(&mut self) -> Deck<'a> {
        self.cards.sort_by(|a, b| a.card_type.cmp(b.card_type));

        let mut pairs = vec![];
        let mut i = 1;

        loop {
            if i >= self.cards.len() {
                break;
            }

            let card1 = &self.cards[i];
            let card2 = &self.cards[i - 1];

            if card1.card_type == card2.card_type {
                let card1 = self.cards.remove(i);
                let card2 = self.cards.remove(i - 1);

                pairs.push(card1);
                pairs.push(card2);
                continue;
            }

            i += 1;
        }

        let mut ret = Deck::empty();
        ret.give_cards(&mut pairs);
        ret
    }

    pub fn find_type(&mut self, card_type: &CardType) -> Option<Card<'a>> {
        for (i, card) in self.cards.iter().enumerate() {
            if card.card_type == card_type {
                let ret = self.cards.remove(i);
                return Some(ret);
            }
        }

        None
    }

    pub fn deal(&mut self, n_cards: usize, players: &mut Vec<&mut Player<'a>>) {
        let len = players.len();
        let cards = self.cards.drain(0..n_cards * len);

        for (i, card) in cards.enumerate() {
            players[i % len].hand.give_card(card);
        }
    }
}

impl<'a> Display for Deck<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let str = self
            .cards
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join(", ");

        fmt.write_fmt(format_args!("[{}]", str))
    }
}

impl<'a> Default for Deck<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Player<'a> {
    pub hand: Deck<'a>,
    pub burn_pile: Deck<'a>,
}

impl<'a> Player<'a> {
    pub fn new() -> Player<'a> {
        let hand = Deck::empty();
        let burn_pile = Deck::empty();
        Player { hand, burn_pile }
    }
}

impl<'a> Default for Player<'a> {
    fn default() -> Self {
        Self::new()
    }
}
