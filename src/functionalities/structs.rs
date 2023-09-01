#![allow(non_snake_case)]

use console::Term;
use console;

use crate::functionalities::errors::{self, ErrorView};

pub static DEPLACEMENT: [i8; 4] = [-1, 1, 1, -1];

pub const WIDTH: usize = 5;
pub const HEIGHT: usize = 5;
pub const LIMIT: [[usize;2];2] = [[0, WIDTH],[0, HEIGHT]];


/// Save the stdin.
pub struct CliEntry {
    // Term is only use to have some pretty color for now.
    pub terminal: Term,
    x: i8,
    y: i8
}

impl CliEntry {
    /// Create a new instance with Default::default() values.
    pub fn new() -> CliEntry {
        CliEntry { terminal: Term::buffered_stdout(),x: Default::default(), y: Default::default() }
    }
    /// A copy of the `x` field.
    pub fn x(&self) -> i8 {
        self.x        
    }
    /// A copy of the `y` field.
    pub fn y(&self) -> i8 {
        self.y
    }
    
    pub fn terminal(&mut self) -> &mut Term {
        &mut self.terminal
    }
}

#[derive(Debug)]
pub struct Board {
    pub state: Vec<Vec<bool>>,
}

/// The game board with a fixed size.
impl Board {
    /// Create a new game board with a fixed size.
    pub fn init() -> Board {
        Board { state: vec![vec![false;WIDTH];HEIGHT] }
    }
    /// Usefull to know the lenght in Y of the matrix.
    pub fn len_height(&self) -> usize {
        self.state.len()
    }
    /// Usefull to know the lenght in X of the matrix.
    pub fn len_width(&self) -> usize {
        match self.state.get(0) {
            Some(raw) => return raw.len(),
            None => panic!("error: out of index"),
        }
    }
    /// Provide a `&mut` of the matrix.
    pub fn board_mut(&mut self) -> &mut Vec<Vec<bool>> {
        &mut self.state
    }
}

/// Store stdin value from CliEntry.
pub struct DataSet {
    // Struct `Move` save the new theorical position
    pub deplc: Move,
    posx: usize,
    posy: usize
}

impl DataSet {
    /// Create a new instance.
    pub fn new() -> DataSet {
        DataSet { deplc: Move { to_x: 0, to_y: 0 }, posx: 0, posy: 0 }
    }
    /// return a copy usize form posx.
    pub fn posx(&self) -> usize {
        self.posx
    }
    /// return a copy usize form posy.
    pub fn posy(&self) -> usize {
        self.posy
    }
    /// return a `&mut` from the value in posx.
    pub fn posx_mut(&mut self) -> &mut usize {
        &mut self.posx
    }
    /// return a `&mut` from the value in posy.
    pub fn posy_mut(&mut self) -> &mut usize {
        &mut self.posy
    }
    /// make transition between the stdin and the program.
    pub fn deplace(&mut self, dep: &[i8], cursor: usize) {
        match cursor {
            0 => { self.deplc.to_y = dep[cursor]; self.deplc.to_x = 0 },
            1 => { self.deplc.to_x = dep[cursor]; self.deplc.to_y = 0 },
            2 => { self.deplc.to_y = dep[cursor]; self.deplc.to_x = 0 },
            3 => { self.deplc.to_x = dep[cursor]; self.deplc.to_y = 0 },
            _ => panic!("error: wrong user entry"),
        }
    }
}

/// deplace idex in the valid valid range or return an printable error to the end user.
pub fn deplc(ds: &mut DataSet, xyLimit: &[[usize; 2]; 2]) -> Result<(), ErrorView> {
    let xyDeplc = (ds.deplc.to_y, ds.deplc.to_x);
    match (xyDeplc.0, xyDeplc.1) {
        // (y, x)
        (-1, 0) if ds.posy > xyLimit[1][0] => { ds.posy -= 1; },
        (0, -1) if ds.posx > xyLimit[0][0] => { ds.posx -= 1; },
        (1, 0) if ds.posy < xyLimit[1][1] - 1 => { ds.posy += 1; },
        (0, 1) if ds.posx < xyLimit[0][1] - 1 => { ds.posx += 1; },
        // no deplacement, invalid behavior
        (0, 0) => panic!("error: can't init a movement"),
        // if the deplacement run out of the limit X, Y
        (_, _) => { let mut err = errors::warn_out_limit(&ds); err.act_context();
            err.add_style_context(console::style("Don't go that way, you'll fall !".to_string()).bold());
            return Err(err) },
    }
    Ok(())
}

/// Save the new theorical position in `DataSet`.
pub struct Move {
    to_x: i8,
    to_y: i8
}

impl Move {
    /// Simple access to x value, return a copy.
    pub fn to_x(&self) -> i8 {
        self.to_x
    }
    /// Simple access to y value, return a copy.
    pub fn to_y(&self) -> i8 {
        self.to_y
    }
    /// An access to x value by a `&mut`.
    pub fn to_mut_x(&mut self) -> &mut i8 {
        &mut self.to_x
    }
    /// An access to y value by a `&mut`.
    pub fn to_mut_y(&mut self) -> &mut i8 {
        &mut self.to_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}