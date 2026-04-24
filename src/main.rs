
struct Board {
    grid: Vec<Vec<bool>>,
}

impl Board {
    /// Creates a new [`Board`].
    fn new(width: usize, height: usize ) -> Self {
        let grid: Vec<Vec<bool>> = vec![vec![false; width]; height];
        Self { grid }
    }

    /// Randomize Board
    fn randomize(&mut self, probability: f64) -> &Self {

        for row in &mut self.grid {
            for cell in row.iter_mut() {
                *cell = rand::random_bool(probability);
            }
        }
        self
    }
}

fn main () {
   let mut g = Board::new( 10, 10 ); 
   g.randomize(0.5);
   println!("{}", g.grid[0][1]);
}