use cards::{CardType, Deck, Player};

use rand::Rng;
use std::io;
use std::{thread, time};

fn read_cmd() -> Option<char> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            let c = buf.chars().next().unwrap();
            Some(c)
        }
        Err(_) => None,
    }
}

fn guess_card_type(input: char) -> Option<CardType> {
    match input {
        'k' => Some(CardType::King),
        'q' => Some(CardType::Queen),
        'j' => Some(CardType::Jack),
        'a' => Some(CardType::Ace),
        '2' => Some(CardType::Number(2)),
        '3' => Some(CardType::Number(3)),
        '4' => Some(CardType::Number(4)),
        '5' => Some(CardType::Number(5)),
        '6' => Some(CardType::Number(6)),
        '7' => Some(CardType::Number(7)),
        '8' => Some(CardType::Number(8)),
        '9' => Some(CardType::Number(9)),
        '1' => Some(CardType::Number(10)),
        _ => None,
    }
}

fn main() {
    let mut deck = Deck::new();
    let mut player1 = Player::new();
    let mut player2 = Player::new();
    let mut random = rand::thread_rng();

    deck.shuffle();
    deck.deal(7, &mut vec![&mut player1, &mut player2]);

    let mut pairs = player1.hand.pairs();
    if !pairs.cards.is_empty() {
        println!("You burn {}", pairs);
        player1.burn_pile.give_deck(&mut pairs);
    }

    let mut pairs = player2.hand.pairs();
    if !pairs.cards.is_empty() {
        println!("Computer burns {} pairs.", pairs.cards.len() / 2);
        player2.burn_pile.give_deck(&mut pairs);
    }

    loop {
        thread::sleep(time::Duration::from_secs(1));

        println!();
        println!("YOUR TURN");
        println!("PLAYER HAND - {}", player1.hand);
        println!("Which card will you ask for?");

        if let Some(input) = read_cmd() {
            if let Some(guess) = guess_card_type(input) {
                println!("You ask for a {}", guess);

                let next_card = match player2.hand.find_type(&guess) {
                    Some(card) => {
                        println!("You get the {}!", card);
                        card
                    }
                    None => match deck.take_card() {
                        Some(next) => {
                            println!("GO FISH! You take the {} from the deck", next);
                            next
                        }
                        None => {
                            println!("GO FISH! No more cards in the deck!");
                            break;
                        }
                    },
                };

                player1.hand.give_card(next_card);

                let mut pairs = player1.hand.pairs();
                if !pairs.cards.is_empty() {
                    println!("You burn {}", pairs);
                    player1.burn_pile.give_deck(&mut pairs);

                    if player1.hand.cards.is_empty() {
                        println!("Your hand is empty!");
                        break;
                    }
                }
            }
        }

        thread::sleep(time::Duration::from_secs(1));
        println!();

        println!("COMPUTER TURN");
        let card_idx = random.gen_range(0..player2.hand.cards.len());
        let random_card = &player2.hand.cards[card_idx];
        let guess_type = random_card.card_type;
        println!("Computer asks for a {}", guess_type);

        let p1_card = match player1.hand.find_type(guess_type) {
            Some(card) => {
                println!("You give the computer the {}", card);
                card
            }
            None => match deck.take_card() {
                Some(next) => {
                    println!("GO FISH! Computer takes a card from the deck");
                    next
                }
                None => {
                    println!("GO FISH! No more cards in the deck!");
                    break;
                }
            },
        };

        player2.hand.give_card(p1_card);

        let mut pairs = player2.hand.pairs();
        if !pairs.cards.is_empty() {
            println!("Computer burns {} pairs.", pairs.cards.len() / 2);
            player2.burn_pile.give_deck(&mut pairs);

            if player2.hand.cards.is_empty() {
                println!("Computer's hand is empty!");
                break;
            }
        }

        println!("Computer has {} cards.", player2.hand.cards.len());

        thread::sleep(time::Duration::from_secs(1));
        println!();
    }

    let player_score = player1.burn_pile.cards.len() / 2;
    let cpu_score = player2.burn_pile.cards.len() / 2;

    println!("Your score: {}", player_score);
    println!("Computer score: {}", cpu_score);

    if player_score > cpu_score {
        println!("YOU WIN!");
    } else {
        println!("COMPUTER WINS!");
    }
}
