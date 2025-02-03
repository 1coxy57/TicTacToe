use std::io::{stdin, stdout, Write};

mod core;
use core::board::{build_board, build_board_with_robot, Board, PlayerType};
use figlet_rs::FIGfont;
use colored::*;


fn main() -> std::io::Result<()> {
    stdout().flush()?;
    let sf = FIGfont::standard().unwrap();
    let fig = sf.convert("coxy.57");
    println!("{}",fig.unwrap().to_string().green());
    println!("{} Play against friend\n{} Play against robot", "[0]".green(),"[1]".green());

    let mut select = String::new();
    stdin().read_line(&mut select)?;
    let num = select.trim().parse::<i32>().unwrap_or(0);

    if num == 0 {
        let mut board = build_board();
        board.show();
        let _players = &board.players;

        loop {
            let curr_player = &board.players[board.current_player];
            let player_s = match curr_player {
                PlayerType::Human(p) => p.value.chars().next().unwrap(),
                _ => unreachable!(), 
            };

            let mut s = String::new();
            print!("[Player: {}] Choose a spot (0-8): ", player_s.to_string().green());
            stdout().flush()?;

            stdin().read_line(&mut s)?;
            let spot: usize = match s.trim().parse() {
                Ok(num) if num < 9 => num,
                _ => {
                    println!("Enter a valid number [0-8]");
                    continue;
                }
            };

            if board.spot_taken(spot) {
                println!("{} already chosen, please pick a valid spot.", spot.to_string().green());
                continue;
            }

            board.choose_spot(spot, player_s);
            board.show();

            if let Some(winner) = board.is_winner() {
                println!("{}! The winner is: {}", "Congrulations".green(), winner.to_string().green());
                return play();
            } else if board.is_tie() {
                println!("{}! No one won! Try again!", "Tie".yellow());
                return play()
            }
        }
    } else {
        println!("[{}] Choose a level (0,1,2) ", "LEVEL".green());

        let mut level = String::new();
        stdout().flush()?;
        stdin().read_line(&mut level)?;
        let level = level.trim().parse::<usize>().unwrap_or(0).clamp(0, 2);

        let mut board = build_board_with_robot(level);
        board.show();

        loop {
            let curr_player = &board.players[board.current_player];

            match curr_player {
                PlayerType::AI(robot) => {
                    println!("[{}] is thinking..", "AI".red());
                    if let Some(spot) = robot.predict(&board) {
                        board.choose_spot(spot, 'O');
                        println!("[{}] chose spot {}", "AI".red(), spot);
                    }
                }
                PlayerType::Human(player) => {
                    let mut s = String::new();
                    print!("[Player: {}] Choose a spot (0-8): ", player.value.to_string().green());
                    stdout().flush()?;

                    stdin().read_line(&mut s)?;
                    let spot: usize = match s.trim().parse() {
                        Ok(num) if num < 9 => num,
                        _ => {
                            println!("Enter a valid number [0-8]");
                            continue;
                        }
                    };

                    if board.spot_taken(spot) {
                        println!("{} already chosen, please pick a valid spot.", spot.to_string().green());
                        continue;
                    }

                    board.choose_spot(spot, player.value.chars().next().unwrap());
                }
            }

            board.show();

            if let Some(winner) = board.is_winner() {
                println!("{}! The winner is: {}", "Congrulations".green(), winner.to_string().green());
                return play();
            } else if board.is_tie() {
                println!("{}! No one won! Try again!", "Tie".yellow());
                return play();
            }
        }
    }
}

fn play() -> std::io::Result<()> {
    println!("Would you like to play again? ({})", "yes/no".green());
    let mut response = String::new();
    stdin().read_line(&mut response)?;
    
    if response.trim().eq_ignore_ascii_case("yes") {
        main() 
    } else {
        Ok(()) 
    }
}
