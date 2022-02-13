use super::{field, validate};

fn change_player(current_player: &mut i32) {
    *current_player = 3 - *current_player;
}

// field, x, y, player
pub fn do_turn(field: &mut [[i32; 3]; 3], x: usize, y: usize, player: &mut i32) -> Result<bool, validate::TileUsedError> {
    match validate::validate_tile(field, x, y) {
        Ok(_) => {
            // play
            field::do_move(field, x, y, *player);
            // check winner
            let winner = field::check_winner(*field, *player);
            // display field
            field::display_field(*field);
            // change player
            change_player(player);

            return Ok(winner);
        }
        Err(e) => {
            return Err(e);
        }
    }
}