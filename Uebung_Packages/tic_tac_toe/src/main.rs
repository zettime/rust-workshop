use std::io::stdin;
use tic_tac_toe_lib::game::{field, validate, controller};
use rand::Rng;
use display_winner as dp;

fn main() {
    let mut field = field::init_field();
    let mut active_player = rand::thread_rng().gen_range(1..3);
    // input: idx1,idx2

    let mut tile_input = String::new();
    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut gameover = false;
    while !gameover {
        println!("Tile (x, y):");
        let mut tile_succ = false;
        while !tile_succ {
            match stdin().read_line(&mut tile_input) {
                Ok(_) => match validate::validate_input(&tile_input) {
                    Ok(coords) => {
                        x = coords.0;
                        y = coords.1;
                        tile_input.clear();
                        tile_succ = true;
                    }
                    Err(e) => println!("Something went wrong: {}", e),
                },
                Err(e) => println!("Something went wrong: {}", e),
            }
        }

        match controller::do_turn(&mut field, x, y, &mut active_player) {
            Ok(winner) => {
                if winner {
                    dp::display_winner(3 - active_player);
                    gameover = true;
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
