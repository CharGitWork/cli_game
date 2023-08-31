use std::cmp::Ordering;

use console::Term;
use console;

use crate::functionalities::errors::{self, ErrorView};

pub static DEPLACEMENT: [i8; 4] = [-1, 1, 1, -1];

pub const WIDTH: usize = 5;
pub const HEIGHT: usize = 5;
pub const LIMIT: [[usize;2];2] = [[0, WIDTH],[0, HEIGHT]];

pub struct CliEntry {
    pub terminal: Term,
    x: i8,
    y: i8
}

impl CliEntry {
    pub fn new() -> CliEntry {
        CliEntry { terminal: Term::buffered_stdout(),x: 0, y: 0 }
    }
    pub fn x(&self) -> i8 {
        self.x        
    }
    pub fn y(&self) -> i8 {
        self.y
    }
}

#[derive(Debug)]
pub struct Board {
    pub state: Vec<Vec<bool>>,
}

impl Board {
    pub fn init() -> Board {
        Board { state: vec![vec![false;WIDTH];HEIGHT] }
    }
    pub fn len_height(&self) -> usize {
        self.state.len()
    }
    pub fn len_width(&self) -> usize {
        match self.state.get(0) {
            Some(raw) => return raw.len(),
            None => panic!("error: out of index"),
        }
    }
    pub fn board_mut(&mut self) -> &mut Vec<Vec<bool>> {
        &mut self.state
    }
}

// #[derive(Eq)]
pub struct DataSet {
    pub deplc: Move,
    posx: usize,
    posy: usize
}

impl DataSet {
    pub fn new() -> DataSet {
        DataSet { deplc: Move { to_x: 0, to_y: 0 }, posx: 0, posy: 0 }
    }
    pub fn posx(&self) -> usize {
        self.posx
    }
    pub fn posy(&self) -> usize {
        self.posy
    }
    pub fn posx_mut(&mut self) -> &mut usize {
        &mut self.posx
    }
    pub fn posy_mut(&mut self) -> &mut usize {
        &mut self.posy
    }
    pub fn deplace(&mut self, dep: &[i8], cursor: usize) {
        match cursor {
            0 => { self.deplc.to_y = dep[cursor]; self.deplc.to_x = 0 },
            1 => { self.deplc.to_x = dep[cursor]; self.deplc.to_y = 0 },
            2 => { self.deplc.to_y = dep[cursor]; self.deplc.to_x = 0 },
            3 => { self.deplc.to_x = dep[cursor]; self.deplc.to_y = 0 },
            _ => panic!("error: wrong user entry"),
        }
    }
    pub fn new_move(&mut self, max: &(usize, usize)) -> Result<(), ErrorView> {
        assert!((self.deplc.to_x | self.deplc.to_y) != 0);
        assert!((max.0 | max.1) != 0);
        assert!(self.posy <= max.0);
        assert!(self.posx <= max.1);

        match self.deplc.to_y {
            1 if self.posy < max.0 => {
                self.posy += 1;
            },
            -1 if self.posy > 0 => {
                self.posy -= 1;
            },
            y if y != 0 => panic!("bug with `y`"),
            to_y => match self.deplc.to_x {
                1 if self.posx < max.1 => {
                    self.posx += 1;
                },
                -1 if self.posx > 0 => {
                    self.posx -= 1;
                },
                x if x != 0 => panic!("bug with `x`"),
                to_x => {
                    let style_err = console::style(format!("incorrect values, y = {} x = {}", to_y, to_x));
                    let style_err = style_err.color256(214);
                    return Err(ErrorView::new_style_err(style_err))
                }
            }
        }
        Ok(())
    }
}

pub struct Move {
    to_x: i8,
    to_y: i8
}

impl Move {
    pub fn to_x(&self) -> i8 {
        self.to_x
    }
    pub fn to_y(&self) -> i8 {
        self.to_y
    }
    pub fn to_mut_x(&mut self) -> &mut i8 {
        &mut self.to_x
    }
    pub fn to_mut_y(&mut self) -> &mut i8 {
        &mut self.to_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}