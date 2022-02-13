use std::fmt;

#[derive(Debug, Clone)]
pub struct TileUsedError;
impl fmt::Display for TileUsedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "specified field already in use! Try again!")
    }
}

pub fn validate_tile(field: &[[i32; 3]; 3], x: usize, y: usize) -> Result<bool, TileUsedError> {
    if field[y][x] != 0 {
        return Err(TileUsedError);
    }
    return Ok(true);
}

#[derive(Debug, Clone)]
pub struct WrongInputError;
impl fmt::Display for WrongInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "specified input wrong! Format: x,y")
    }
}

pub fn validate_input(input: &String) -> Result<(usize, usize), WrongInputError> {
    let coords: Vec<&str> = input.split(",").collect();
    if coords.len() == 2 {
        let x: usize;
        let y: usize;

        match str::parse::<usize>(coords[0].trim()) {
            Ok(val) => {
                x = val;
            }
            Err(_) => {
                return Err(WrongInputError);
            }
        };

        match str::parse::<usize>(coords[1].trim()) {
            Ok(val) => {
                y = val;
            }
            Err(_) => {
                return Err(WrongInputError);
            }
        };

        if x < 3 && y < 3 {
            return Ok((x, y));
        }
    }
    return Err(WrongInputError);
}
