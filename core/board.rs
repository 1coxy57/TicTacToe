use rand::Rng;

#[derive(Clone)]
pub struct Player {
    pub value: String,
}

#[derive(Clone)]
pub struct Robot {
    level: usize,
}

#[derive(Clone)]
pub enum PlayerType {
    Human(Player),
    AI(Robot),
}

#[derive(Clone)]
pub struct Board {
    pub players: Vec<PlayerType>,
    pub grid: Vec<char>,
    pub current_player: usize,
}


impl Robot {
    pub fn predict(&self, board: &Board) -> Option<usize> {
        match self.level {
            0 => self.basic_predict(board),
            1 => self.predict_1(board, true).1,
            _ => Some(0),
        }
    }

    fn basic_predict(&self,board: &Board) ->  Option<usize> {
        board.grid.iter().position(|&c| c == ' ')
    }

    fn predict_1(&self, board: &Board, m: bool) -> (i32, Option<usize>) {
        if let Some(winner) = board.is_winner() {
            return match winner.as_str() {
                "X" => (1, None),
                "O" => (-1, None),
                _ => (0, None),
            };
        }
        if board.is_tie() {
            return (0, None);
        }

        let mut bscr = if m { i32::MIN } else { i32::MAX };
        let mut bmve = None;
        let p_char = if m { 'X' } else { 'O' };

        for (i, &c) in board.grid.iter().enumerate() {
            if c == ' ' {
                let mut new_board = board.clone();
                new_board.grid[i] = p_char;
                let (score, _) = self.predict_1(&new_board, !m);
                
                if m {
                    if score > bscr {
                        bscr = score;
                        bmve = Some(i);
                    }
                } else {
                    if score < bscr {
                        bscr = score;
                        bmve = Some(i);
                    }
                }
            }
        }

        (bscr, bmve)
    }
}

impl Board {
    pub fn choose_spot(&mut self, index: usize, ch: char) -> bool {
        if index < 9 && self.grid[index] == ' ' {
            self.grid[index] = ch;
            self.switch_curr_player();
            true
        } else {
            false
        }
    }

    pub fn get_curr_player(&self) -> &PlayerType {
        &self.players[self.current_player]
    }

    pub fn show(&self) {
        for (i, row) in self.grid.chunks(3).enumerate() {
            let rs = row.iter().collect::<String>();
            println!("{}", rs);
            if i < 2 {
                println!("---");
            }
        }
    }

    pub fn spot_taken(&self, spot: usize) -> bool {
        self.grid[spot] != ' '
    }

    pub fn is_tie(&self) -> bool {
        self.grid.iter().all(|&c| c != ' ') && self.is_winner().is_none()
    }

    pub fn switch_curr_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }

    pub fn is_winner(&self) -> Option<String> {
        let patterns = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], 
            [0, 3, 6], [1, 4, 7], [2, 5, 8], 
            [0, 4, 8], [2, 4, 6], 
        ];
        patterns.iter().find_map(|&[f, s, t]| {
            let o = self.grid[f]; 
            if o != ' ' && o == self.grid[s] && o == self.grid[t] {
                Some(o.to_string())
            } else {
                None
            }
        })
    }
}

fn create_players() -> Vec<PlayerType> {
    vec![
        PlayerType::Human(Player { value: "X".to_string() }),
        PlayerType::Human(Player { value: "O".to_string() }),
    ]
}

pub fn build_board() -> Board {
    let players = create_players();
    let current_player = rand::thread_rng().gen_range(0..players.len());
    Board {
        players,
        grid: vec![' '; 9],
        current_player,
    }
}

pub fn build_board_with_robot(level: usize) -> Board {
    let robot = Robot { level };
    let players = vec![
        PlayerType::Human(Player { value: "X".to_string() }),
        PlayerType::AI(robot),
    ];
    let current_player = rand::thread_rng().gen_range(0..players.len());
    
    Board {
        players,
        grid: vec![' '; 9],
        current_player,
    }
}
