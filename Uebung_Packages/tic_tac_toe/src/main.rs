use std::io::stdin;
use tic_tac_toe_lib::game::{field, validate, controller};

fn main() {
    let mut field = field::init_field();
    let mut active_player = 1;
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
                    println!("Winner is player: {}", 3 - active_player);
                    gameover = true;
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
