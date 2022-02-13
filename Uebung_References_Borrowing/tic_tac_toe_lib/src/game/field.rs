pub fn init_field() -> [[i32; 3]; 3] {
    [[0; 3]; 3]
}

// TODO:
// signature: display_field(field)
// should be immediatly printed to console
// pub fn display_field...

// TODO:
// signature: do_move(field, x, y, player)
// field should be changed in place 
// pub fn do_move...

pub fn check_winner(field: [[i32; 3]; 3], player: i32) -> bool {
    for i in 0..3 {
        if check_cols(field, i, player) {
            return true;
        }
    }

    for i in 0..3 {
        if check_rows(field, i, player) {
            return true;
        }
    }

    if check_diag(field, true, player) {
        return true;
    }

    return check_diag(field, false, player);
}

fn check_cols(field: [[i32; 3]; 3], x: usize, player: i32) -> bool {
    for y in 0..3 {
        if player != field[y][x] {
            return false;
        }
    }

    return true;
}

fn check_rows(field: [[i32; 3]; 3], y: usize, player: i32) -> bool {
    for x in 0..3 {
        if field[y][x] != player {
            return false;
        }
    }

    return true;
}

// diag = 0 or 1
fn check_diag(field: [[i32; 3]; 3], diag_lr: bool, player: i32) -> bool {
    if diag_lr {
        for i in 0..3 {
            if field[i][i] != player {
                return false;
            }
        }
    } else {
        for i in 0..3 {
            if field[i][2 - i] != player {
                return false;
            }
        }
    }
    return true;
}