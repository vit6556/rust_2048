use std::io;
use std::io::Write;

pub struct Board {
    size: usize,
    max_value: usize,
    board: Vec<Vec<usize>>,
}

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