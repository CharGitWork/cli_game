pub mod functionalities;
pub mod prelude;
pub mod tests;

use crate::functionalities::structs::Board;

pub fn view(dis: &Board, mut f: impl std::io::Write) {
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