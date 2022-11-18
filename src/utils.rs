use crate::types::CardType;
use std::io;

pub fn read_cmd() -> Option<char> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            let c = buf.chars().next().unwrap();
            Some(c)
        }
        Err(_) => None,
    }
}

pub fn card_type_char(input: char) -> Option<CardType> {
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

pub fn guess_card_type() -> Option<CardType> {
    match read_cmd() {
        Some(ch) => card_type_char(ch),
        None => None,
    }
}
