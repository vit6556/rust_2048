use rand::Rng;
use crate::game::board::Board;

pub struct Game {
    board: Board,
    score: usize,
}

impl Game {
    pub fn start() -> Self {
        let mut game = Self {
            board: Board::new(4),
            score: 0,
        };

        game.put_new_value();
        game.put_new_value();
        game
    }

    fn put_new_value(&mut self) {
        let cords: (usize, usize) = loop {
            let x = rand::thread_rng().gen_range(0..=3);
            let y = rand::thread_rng().gen_range(0..=3);

            if self.board.get_value(x, y) == 0 {
                break (x, y);
            }
        };

        let value: usize = match rand::thread_rng().gen_range(0..=9) {
            0 => 4,
            _ => 2,
        };

        self.board.set_value(cords.0, cords.1, value);
    }

    pub fn show_board(&self) {
        //print!("\x1B[2J\x1B[1;1H");
        println!("Score: {}", self.score);
        self.board.show();
    }
}

