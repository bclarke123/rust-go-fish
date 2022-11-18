use cards::{CardType, Deck, Player};

use rand::Rng;
use std::io;
use std::{thread, time};

fn read_cmd() -> Option<String> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => Some(buf),
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

    let mut players = vec![&mut player1, &mut player2];

    deck.shuffle();
    deck.deal(7, &mut players);

    let mut random = rand::thread_rng();

    loop {
        let mut pairs = player1.hand.pairs();
        println!("You burn {}", pairs);
        player1.burn_pile.give_deck(&mut pairs);

        if player1.hand.cards.len() == 0 {
            println!("YOU WIN!");
            break;
        }

        let mut pairs = player2.hand.pairs();
        println!(
            "Computer burns {} pairs.  Computer has {} cards.",
            pairs.cards.len() / 2,
            player2.hand.cards.len()
        );
        player2.burn_pile.give_deck(&mut pairs);

        if player2.hand.cards.len() == 0 {
            println!("COMPUTER WINS!");
            break;
        }

        println!("PLAYER HAND - {}", player1.hand);
        println!("Which card will you ask for?");

        if let Some(cmd) = read_cmd() {
            let input = cmd.chars().next().unwrap();
            if let Some(guess) = guess_card_type(input) {
                println!("You ask for a {}", guess);

                let next_card = match player2.hand.find_type(&guess) {
                    Some(card) => {
                        println!("You get the {}!", card);
                        card
                    }
                    None => {
                        let next = deck.take_card();
                        println!("GO FISH! You take the {} from the deck", next);
                        next
                    }
                };

                player1.hand.give_card(next_card);
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
            None => {
                let next = deck.take_card();
                println!("GO FISH! Computer takes a card from the deck");
                next
            }
        };

        player2.hand.give_card(p1_card);

        thread::sleep(time::Duration::from_secs(1));
        println!();
    }
}
