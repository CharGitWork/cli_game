use cli::functionalities::structs::DEPLACEMENT;
use cli::prelude::*;

use console::*;

use cli::functionalities::structs::LIMIT;

fn main() {
    const XY_LIMIT: [[usize;2];2] = LIMIT;
    let prog_entry = CliEntry::new();
    let mut gameboard = Board::init();
    let mut player = DataSet::new();

    let height = gameboard.len_height();
    let width = gameboard.len_width();
    let mut f = std::io::stdout().lock();

    *player.posx_mut() = width / 2;
    *player.posy_mut() = height / 2;

    (*gameboard.board_mut())[player.posy()][player.posx()] = true;

    cli::view(&gameboard, &mut f);

    'game_loop:loop {
        'user_entry:loop {
            let index: usize = match prog_entry.terminal.read_key() {
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
        
        if let Err(error) = player.new_move(&(height, width)) {
            eprintln!("{}", error);
            break 'game_loop;
        }

        (*gameboard.board_mut())[player.posy()][player.posx()] = true;
        cli::view(&gameboard, &mut f);
    }
}