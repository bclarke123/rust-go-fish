use crate::types::{CardGame, Deck, Player};
use crate::utils::guess_card_type;
use rand::rngs::ThreadRng;
use rand::Rng;

fn burn<F>(player: &mut Player, cb: F)
where
    F: Fn(&Deck),
{
    let mut pairs = player.hand.pairs();
    if !pairs.cards.is_empty() {
        cb(&pairs);
        player.burn_pile.give_deck(&mut pairs);
    }
}

pub struct GoFish<'a> {
    deck: Deck<'a>,
    player1: Player<'a>,
    player2: Player<'a>,
    random: ThreadRng,
}

impl<'a> GoFish<'a> {
    pub fn new() -> Self {
        let deck = Deck::new();
        let player1 = Player::new();
        let player2 = Player::new();
        let random = rand::thread_rng();

        Self {
            deck,
            player1,
            player2,
            random,
        }
    }
}

impl<'a> CardGame for GoFish<'a> {
    fn init(&mut self) {
        self.deck.shuffle();
        self.deck
            .deal(7, &mut vec![&mut self.player1, &mut self.player2]);

        burn(&mut self.player1, |pairs| println!("You burn {}", pairs));

        burn(&mut self.player2, |pairs| {
            println!("Computer burns {} pairs.", pairs.cards.len() / 2)
        });
    }

    fn next_player_turn(&mut self) -> bool {
        println!("YOUR TURN");
        println!("YOUR HAND - {}", self.player1.hand);
        println!("Computer has {} cards", self.player2.hand.cards.len());
        println!("Which card will you ask for?");

        if let Some(guess) = guess_card_type() {
            println!("You ask for a {}", guess);

            let next_card = match self.player2.hand.find_type(&guess) {
                Some(card) => {
                    println!("You get the {}!", card);
                    card
                }
                None => match self.deck.take_card() {
                    Some(next) => {
                        println!("GO FISH! You take the {} from the deck", next);
                        next
                    }
                    None => {
                        println!("GO FISH! No more cards in the deck!");
                        return false;
                    }
                },
            };

            self.player1.hand.give_card(next_card);

            burn(&mut self.player1, |pairs| println!("You burn {}", pairs));

            if self.player1.hand.cards.is_empty() {
                println!("Your hand is empty!");
                return false;
            }
        }
        true
    }

    fn next_computer_turn(&mut self) -> bool {
        println!("COMPUTER TURN");
        println!("Computer has {} cards.", self.player2.hand.cards.len());

        let n_cards = self.player2.hand.cards.len();
        let card_idx = match n_cards {
            1 => 0,
            _ => self.random.gen_range(0..n_cards),
        };
        let random_card = &self.player2.hand.cards[card_idx];
        let guess_type = random_card.card_type;
        println!("Computer asks for a {}", guess_type);

        let p1_card = match self.player1.hand.find_type(guess_type) {
            Some(card) => {
                println!("You give the computer the {}", card);
                card
            }
            None => match self.deck.take_card() {
                Some(next) => {
                    println!("GO FISH! Computer takes a card from the deck");
                    next
                }
                None => {
                    println!("GO FISH! No more cards in the deck!");
                    return false;
                }
            },
        };

        self.player2.hand.give_card(p1_card);

        burn(&mut self.player2, |pairs| {
            println!("Computer burns {} pairs.", pairs.cards.len() / 2)
        });

        if self.player2.hand.cards.is_empty() {
            println!("Computer's hand is empty!");
            return false;
        }

        true
    }

    fn game_over(&self) {
        let player_score = self.player1.burn_pile.cards.len() / 2;
        let cpu_score = self.player2.burn_pile.cards.len() / 2;

        println!();
        println!("GAME OVER!!");
        println!();

        println!("Your score: {}", player_score);
        println!("Computer score: {}", cpu_score);

        if player_score > cpu_score {
            println!("YOU WIN!");
        } else {
            println!("COMPUTER WINS!");
        }
    }
}
