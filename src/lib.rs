use std::io;

use console::Term;

pub static DEPLACEMENT: [i8; 4] = [-1, 1, 1, -1];

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
        Board { state: vec![vec![false;5];5] }
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

pub struct DataSet {
    deplc: Move,
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
    pub fn new_move(&mut self, max: (usize, usize)) -> Result<(), String> {
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
                to_x => return Err(format!("incorrect values, y = {} x = {}", to_y, to_x))
            }
        }
        Ok(())
    }
}

pub fn view(dis: &Board, mut f: impl io::Write) {
    dis.state
        .iter().for_each(|v| {
            v.iter().enumerate().for_each(|(i, b)| {
                match b {
                    false if i % 5 != 0 => {
                        write!(f, "[0]").unwrap();
                    },
                    false => {
                        write!(f, "\n[0]").unwrap();
                    },
                    true if i % 5 != 0 => {
                        write!(f, "(X)").unwrap();
                    },
                    true => {
                        write!(f, "\n(X)").unwrap()
                    },
                };
            })
        });
    writeln!(f, "\n").unwrap();
}

// ---------------------------- //

struct Move {
    to_x: i8,
    to_y: i8
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn move_to_the_right() {
        let mut plyr = DataSet::new();
        plyr.posx = 5;
        plyr.deplc.to_x = 1;
        let max_x = 6;
        plyr.new_move((3, max_x)).unwrap();
        assert_eq!(plyr.posx, max_x);
    }
    #[test]
    fn move_to_the_left() {
        let mut plyr = DataSet::new();
        plyr.posx = 1;
        plyr.deplc.to_x = -1;
        plyr.new_move((3, 6)).unwrap();
        assert_eq!(plyr.posx, 0);
    }
    #[test]
    fn move_to_the_bottom() {
        let mut plyr = DataSet::new();
        plyr.posy = 2;
        plyr.deplc.to_y = 1;
        let max_y = 3;
        plyr.new_move((max_y, 6)).unwrap();
        assert_eq!(plyr.posy, max_y);
    }
    #[test]
    fn move_to_the_up()  {
        let mut plyr = DataSet::new();
        plyr.posy = 1;
        plyr.deplc.to_y = -1;
        let max_y = 3;
        plyr.new_move((max_y, 6)).unwrap();
        assert_eq!(plyr.posy, 0);
    }
    #[test]
    fn new_move_to_x_non_eq_zero() {
        let mut plyr = DataSet::new();
        // to_y == 0
        plyr.deplc.to_x = 1;
        let max_y = 3;
        let max_x = 6;
        if let Err(e) = plyr.new_move((max_y, max_x)) {
            panic!("{}", e);
        }
    }
    #[test]
    fn new_move_to_y_non_eq_zero() {
        let mut plyr = DataSet::new();
        // to_x == 0
        plyr.deplc.to_y = 1;
        let max_y = 3;
        let max_x = 6;
        if let Err(e) = plyr.new_move((max_y, max_x)) {
            panic!("{}", e);
        }
    }
    #[test]
    #[should_panic]
    fn new_move_aways_from_zero() {
        let mut plyr = DataSet::new();
        plyr.posy = 0;
        plyr.deplc.to_y = -1;
        let max_y = 3;
        let max_x = 6;
        if let Ok(_) = plyr.new_move((max_y, max_x)) {
            panic!("y out of index");
        }
        plyr.deplc.to_y = 0;

        plyr.posx = 0;
        plyr.deplc.to_x = -1;
        if let Err(err) = plyr.new_move((max_y, max_x)) {
            panic!("{}", err);
        }
    }
    #[test]
    #[should_panic]
    fn new_move_aways_from_max_limit() {
        let mut plyr = DataSet::new();
        let max_y = 3;
        let max_x = 6;

        plyr.posy = max_y;
        plyr.deplc.to_y = 1;

        if let Ok(_) = plyr.new_move((max_y, max_x)) {
            panic!("y out of index");
        }
        plyr.deplc.to_y = 0;

        plyr.posx = max_x;
        plyr.deplc.to_x = 1;
        if let Err(err) = plyr.new_move((max_y, max_x)) {
            panic!("{}", err);
        }
    }

    #[test]
    #[should_panic]
    fn call_new_move_where_to_x_and_to_y_eq_zero() {
        let mut plyr = DataSet::new();
        let max_y = 3;
        let max_x = 6;

        // to_x and to_y == 0

        if let Err(_) = plyr.new_move((max_y, max_x)) {
            panic!("y and x == 0");
        }
    }

    #[test]
    fn deplace_up() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 0;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_y, -1);
    }
    #[test]
    fn deplace_right() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 1;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_x, 1);
    }
    #[test]
    fn deplace_down() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 2;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_y, 1);
    }
    #[test]
    fn deplace_left() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 3;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_x, -1);
    }
    #[test]
    #[should_panic]
    fn deplace_index_to_high() {
        let mut p = DataSet::new();
        let dep = DEPLACEMENT;
        let index = 4;

        p.deplace(&dep, index);
        assert_eq!(p.deplc.to_x, -1);
    }
}