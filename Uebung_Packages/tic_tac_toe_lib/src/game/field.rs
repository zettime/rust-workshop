pub fn init_field() -> [[i32; 3]; 3] {
    [[0; 3]; 3]
}

pub fn display_field(field: [[i32; 3]; 3]) {
    let mut display = String::new();
    for y in 0..3 {
        for x in 0..3 {
            display.push_str(field[y][x].to_string().as_str());
            if x <= 1 {
                display.push_str(" | ");
            }
        }
        display.push_str("\n");
    }
    println!("{}", display);
}

pub fn do_move(field: &mut [[i32; 3]; 3], x: usize, y: usize, player: i32) {
    field[y][x] = player;
}

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