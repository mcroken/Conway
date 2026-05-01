//use std::io;
use std::collections::HashMap;
use proceed::any_or_quit_with;

struct Board {
    grid: HashMap<(i64,i64),bool>,
    width: i64,
    height: i64,
}

impl Board {
    /// Creates a new [`Board`].
    fn new ( ) -> Self {
        let grid = HashMap::new();
        let height: i64 = 0;
        let width: i64 = 0;
        Self { grid, height, width }
    }
    // Look into the "Builder Pattern" for optional parameters
    fn random_board ( &mut self, width: i64, height: i64 ) {
        let x_origin: i64 = 0;
        let y_origin: i64 = 0;
        self.width = width;
        self.height = height;
        for x in x_origin..width {
            for y in y_origin..height {
                let point = (x,y);
                let state: bool = rand::random();
                self.grid.insert(point,state);
            }
        }
    }

    fn display ( &self, width: i64, height: i64 ) {
        let x_origin: i64 = 0;
        let y_origin: i64 = 0;
        for y in (y_origin..height).rev() {
            let mut row = Vec::new();
            for x in x_origin..width {
                let coordinate: (i64,i64) = (x,y);
                let state: bool = self.grid.get( &coordinate ).copied().unwrap_or(false);
                if state {
                    row.push('O');
                } else {
                    row.push(' ');
                }
            }
            println!("{:?}",row)
        }
    }

    fn count_neighbors ( &self, coordinate: (i64,i64) ) -> i8 {
        let (x,y) = coordinate;
        let mut live_count: i8 = 0;
        for i in x-1..x+2{
            for j in y-1..y+2{
                if i == x && j == y {
                    continue;
                }
                let neighbor= (i,j);
                let state: bool = self.grid.get( &neighbor ).copied().unwrap_or(false);
                if state {
                    live_count += 1;
                }
            }
        }
        live_count
    }

    // Needs min-max logic to check boundary coords
    fn cycle ( &self ) -> Board {
        let mut next_generation: Board = Board::new();

        for (coord, state) in &self.grid {
            let live_count = self.count_neighbors( *coord );
            if *state {
                if live_count == 2 || live_count == 3 {
                    next_generation.grid.insert ( *coord, true );
                } else {
                    next_generation.grid.insert ( *coord, false );
                }
            } else {
                if live_count == 3 {
                    next_generation.grid.insert ( *coord, true );
                } else {
                    next_generation.grid.insert ( *coord, false );
                }
            }
        }
        next_generation
    }

}

fn main () {
    let width = 10;
    let height = 10;
    let mut my_board = Board::new();
    my_board.random_board(width,height);
    // Loop cycle calls, pause for input betweeen each iter.
    loop {
        my_board.display(width,height);
        if !any_or_quit_with('q'){
            break;
        }
        my_board = my_board.cycle();
    }
}