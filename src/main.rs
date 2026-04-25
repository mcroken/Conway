use std::io;

struct Board {
    grid: Vec<Vec<char>>,
}

impl Board {
    /// Creates a new [`Board`].
    fn new(width: usize, height: usize ) -> Self {
        let grid: Vec<Vec<char>> = vec![vec!['O'; width]; height];
        Self { grid }
    }

    /// Randomize Board
    fn randomize(&mut self, probability: f64) -> &Self {

        for row in &mut self.grid {
            for cell in row.iter_mut() {
                let alive: bool = rand::random_bool(probability);
                if alive {
                    *cell = 'X';
                } else {
                    *cell = 'O';
                }
            }
        }
        self
    }

    fn print_board(&mut self) {
        for row in &self.grid {
            println!("{:?}",row);
        }
    }
}

fn main () {
    let mut run: bool = true;
    let mut input: String = String::new();
    let stdin = io::stdin();
    let mut g = Board::new( 10, 10 );
    g.randomize(0.5);
    while run {
        g.print_board();
        stdin.read_line(&mut input);
        run = false;
    }
}