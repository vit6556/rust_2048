use std::io;
use std::io::Write;

pub enum Moves {
    Up,
    Down,
    Left,
    Right,
}

pub struct Board {
    size: usize,
    max_value: usize,
    board: Vec<Vec<usize>>,
}

// Basic impl
impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            board: vec![vec![0; size]; size],
            max_value: 0,
            size: size,
        }
    }

    pub fn get_value(&self, x: usize, y: usize) -> usize {
        self.board[x][y]
    }

    fn update_max_value(&mut self, new_value: usize) {
        if new_value > self.max_value {
            self.max_value = new_value;
        }
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: usize) {
        self.board[x][y] = value;
        self.update_max_value(value);
    }

    pub fn check_board_full(&self) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }

        true
    }
}

// Moves impl
impl Board {
    pub fn process_move(&mut self, m: Moves) -> usize {
        let score = match m {
            Moves::Up => self.move_up(),
            Moves::Down => self.move_down(),
            Moves::Left => self.move_left(),
            Moves::Right => self.move_right(),
        };
        score
    }

    fn move_up(&mut self) -> usize {
        let mut score = 0;
        for j in 0..4 {
            let mut li: usize = 0;
            let ri: usize = j;
            for i in 1..4 {
                if self.board[i][j] != 0 {
                    if self.board[i - 1][j] == 0 || self.board[i - 1][j] == self.board[i][j] {
                        if self.board[li][ri] == self.board[i][j] {
                            self.board[li][ri] *= 2;
                            score += self.board[li][ri];
                        } else {
                            if self.board[li][ri] != 0 {
                                li += 1;
                            }
                            self.board[li][ri] = self.board[i][j];
                        }
                        self.board[i][j] = 0;
                    } else {
                        li += 1;
                    }
                }
            }
        }
        score
    }

    fn move_down(&mut self) -> usize{
        let mut score = 0;
        for j in 0..4 {
            let mut li: usize = 3;
            let ri: usize = j;
            for i in (0..=2).rev() {
                if self.board[i][j] != 0 {
                    if self.board[i + 1][j] == 0 || self.board[i + 1][j] == self.board[i][j] {
                        if self.board[li][ri] == self.board[i][j] {
                            self.board[li][ri] *= 2;
                            score += self.board[li][ri];
                        } else {
                            if self.board[li][ri] != 0 {
                                li -= 1;
                            }
                            self.board[li][ri] = self.board[i][j];
                        }
                        self.board[i][j] = 0;
                    } else {
                        li -= 1;
                    }
                }
            }
        }
        score
    }

    fn move_left(&mut self) -> usize {
        let mut score = 0;
        for i in 0..4 {
            let li: usize = i;
            let mut ri: usize = 0;
            for j in 1..4 {
                if self.board[i][j] != 0 {
                    if self.board[i][j - 1] == 0 || self.board[i][j - 1] == self.board[i][j] {
                        if self.board[li][ri] == self.board[i][j] {
                            self.board[li][ri] *= 2;
                            score += self.board[li][ri];
                        } else {
                            if self.board[li][ri] != 0 {
                                ri += 1;
                            }
                            self.board[li][ri] = self.board[i][j];
                        }
                        self.board[i][j] = 0;
                    } else {
                        ri += 1;
                    }
                }
            }
        }
        score
    }

    fn move_right(&mut self) -> usize{
        let mut score = 0;
        for i in 0..4 {
            let li: usize = i;
            let mut ri: usize = 3;
            for j in (0..=2).rev() {
                if self.board[i][j] != 0 {
                    if self.board[i][j + 1] == 0 || self.board[i][j + 1] == self.board[i][j] {
                        if self.board[li][ri] == self.board[i][j] {
                            self.board[li][ri] *= 2;
                            score += self.board[li][ri];
                        } else {
                            if self.board[li][ri] != 0 {
                                ri -= 1;
                            }
                            self.board[li][ri] = self.board[i][j];
                        }
                        self.board[i][j] = 0;
                    } else {
                        ri -= 1;
                    }
                }
            }
        }
        score
    }
}

// Show board impl
impl Board {
    fn get_value_length(&self, current_value: usize) -> usize {
        let mut max_length = 0;
        let mut current_value = current_value;
        while current_value > 0 {
            current_value /= 10;
            max_length += 1;
        }
        max_length
    }

    pub fn show(&self) {
        let max_length = self.get_value_length(self.max_value);

        for i in 0..self.size {
            println!("{:-<1$}", "", (max_length + 2) * self.size + 1);

            for j in 0..self.size {
                let value = self.board[i][j];
                if value == 0 {
                    print!("|{: <1$}", "", max_length + 1);
                } else {
                    print!("|{}", value);
                    print!("{: <1$}", "", max_length + 1 - self.get_value_length(value));
                }
            }
            print!("|\n");
        }
        println!("{:-<1$}", "", (max_length + 2) * self.size + 1);

        io::stdout().flush().unwrap();
    }
}
