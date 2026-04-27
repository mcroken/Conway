//use std::io;
use std::collections::HashMap;

struct Board {
    grid: HashMap<(i64,i64),bool>,
}

impl Board {
    /// Creates a new [`Board`].
    fn new( ) -> Self {
        let grid = HashMap::new();
        Self { grid }
    }
    // Look into the "Builder Pattern" for optional parameters
    fn random_board ( &mut self, width: i64, height: i64 ) {
        let x_origin: i64 = 0;
        let y_origin: i64 = 0;
        for x in x_origin..width {
            for y in y_origin..height {
                let point = (x,y);
                let state: bool = rand::random();
                self.grid.insert(point,state);
            }
        }
    }

    fn dissect (self) {
        for (k,v) in self.grid {
            println!("{:?}: {:?}", k, v);
        }
    }

}

fn main () {
    let mut my_board = Board::new();
    my_board.random_board(10,12);
    my_board.dissect();
}