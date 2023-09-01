pub mod functionalities;
pub mod prelude;
pub mod tests;

use console::Term;

use crate::functionalities::structs::Board;

pub fn view(dis: &Board, f: &mut Term) {
    let mut lines = String::with_capacity(80);

    dis.state.iter().flatten().enumerate().for_each(|(i, b)| {
                match b {
                    false if !(i % 5 == 0) => {
                        lines.push_str("[0]");
                    },
                    false => {
                        lines.push_str("\n[0]");
                    },
                    true if !(i % 5 == 0)=> {
                        lines.push_str("(X)");
                    },
                    true => {
                        lines.push_str("\n(X)");
                    },
                };
            }
        );
    f.write_line(&lines).expect("bug in cli::view()");
    f.flush().expect("bug in cli::view()");
}