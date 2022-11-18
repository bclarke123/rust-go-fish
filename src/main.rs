use cards::go_fish::GoFish;
use cards::types::CardGame;
use cards::utils::read_cmd;
use std::{thread, time};

fn main() {
    loop {
        let mut game = GoFish::new();
        game.init();

        println!();

        loop {
            if !game.next_player_turn() {
                break;
            }

            thread::sleep(time::Duration::from_secs(1));
            println!();

            if !game.next_computer_turn() {
                break;
            }

            thread::sleep(time::Duration::from_secs(1));
            println!();
        }

        game.game_over();

        println!("Play again? y/n");
        let quit = matches!(read_cmd(), Some('n'));

        if quit {
            break;
        }
    }
}
