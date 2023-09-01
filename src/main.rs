#![allow(non_upper_case_globals)]

use cli::functionalities::structs::DEPLACEMENT;
use cli::prelude::*;

use console::*;

use cli::functionalities::structs::{self, LIMIT};

fn main() {
    const xyLimit: [[usize;2];2] = LIMIT;
    let mut console = CliEntry::new();
    let mut gameboard = Board::init();
    let mut player = DataSet::new();

    let height = gameboard.len_height();
    let width = gameboard.len_width();
    // let mut f = std::io::stdout().lock();

    *player.posx_mut() = width / 2;
    *player.posy_mut() = height / 2;

    (*gameboard.board_mut())[player.posy()][player.posx()] = true;

    cli::view(&gameboard, console.terminal());

    'game_loop:loop {
        'user_entry:loop {
            let index: usize = match console.terminal.read_key() {
                Ok(key) => match key {
                    Key::ArrowUp => 0,
                    Key::ArrowRight => 1,
                    Key::ArrowDown => 2,
                    Key::ArrowLeft => 3,
                    Key::Char('q') => break 'game_loop,
                    _ => continue 'user_entry
                },
                Err(error) => {
                    eprintln!("{}", error);
                    continue 'user_entry
                }
            };
            player.deplace(&DEPLACEMENT, index);
            break 'user_entry
        }

        (*gameboard.board_mut())[player.posy()][player.posx()] = false;
        
        if let Err(error) = structs::deplc(&mut player, &xyLimit) {
            eprintln!("{}", error);
            match console.terminal.read_key() {
                Ok(key) => match key {
                    Key::Char('q') => break 'game_loop,
                    _ => console.terminal().clear_screen().expect("bug to clean stdout"),
                }
                Err(error) => panic!("error caused by: {}", error),
            }
        }
        // temp
        console.terminal.clear_screen().expect("bug to clean stdout");
        console.terminal.write_line("push 'q' to leave").expect("bug writing on stdout in main function");

        (*gameboard.board_mut())[player.posy()][player.posx()] = true;
        
        cli::view(&gameboard, console.terminal());
    }
}