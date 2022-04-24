use std::env;
use std::collections::HashMap;


struct Board {
    vec: Vec<Vec<char>>,
}


fn init_board(set_rows: usize, set_cols: usize) -> Board{
    let board_vector = vec![vec![ '#'; set_rows]; set_cols];
    Board {
        vec: board_vector,
    }
}


impl Board {

    fn print_board(&mut self){
        for row in &self.vec{
            println!("{:?}", row);
        }
    }

    fn place_piece(&mut self, row: usize, col: usize, piece: char){
        self.vec[row][col] = piece;
    }

    fn position_map(self) {
        let mut position_map: HashMap<i32, (i32, i32)>;
        //return position_map  
    }


    fn check_diagonals(&mut self, piece: char) -> bool {
        let mut left_diag = true;
        let mut right_diag = true;
        let mut count = 0;
        let mut temp = self.vec.len() - 1;

        loop {

            if count == self.vec.len() - 1 {
                break;
            }
            if self.vec[count][temp] != piece {
                right_diag = false;
            }
            if self.vec[count][count] != piece {
                left_diag = false;
            }

            temp -= 1;
            count += 1
        }

        if left_diag == true || right_diag == true {
            return true
        }
        return false

    }


	fn check_vert_horiz(self, piece: char) -> bool {
        let mut horiz = true;
        let mut vert = true;
        let mut row_index = 0;
        let mut col_index = 0;
        for row in &self.vec {
            let mut temp_horiz = true;
            let mut temp_vert = true;
            for col in row {
                if *col != piece {
                    temp_horiz = false;
                }
                if self.vec[col_index][row_index] != piece {
                    temp_vert = false;
                }
                col_index += 1;
            }

            if temp_horiz == true || temp_vert == true {
                break;
            } else {
                horiz = false;
                vert = false;
            }
            row_index += 1;
            col_index = 0;
        }

        if horiz == true || vert == true {
            return true
        }
        return false
    }
}	


// validate rows and columns CLI inputs for board dimensions
fn validate_row_cols(row_value: &String, col_value: &String) -> (usize, usize){
    // validate row as integer

    let row: usize = match row_value.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("Row value is not an integer ");
            return (0, 0)
        }
    };

    // validate col as integer
    let col: usize = match col_value.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("Col value is not an integer ");
            return (0, 0)
        }
    };

    (row, col)

}



fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    match args.len() {
        1 => {
            println!("No arguments passed ");
        },
        2 => {
            println!("Row specified no colums ");
        },
        3 => {
            println!("Row and col specified ");
            let row_value = &args[1];
            let col_value = &args[2];
            let val_tuple = validate_row_cols(&row_value, &col_value);

            let mut board = init_board(val_tuple.0, val_tuple.1);
            board.print_board();
            println!("===================");
            board.place_piece(0, 0, 'X');
            board.place_piece(0, 1, 'X');
            board.place_piece(1, 2, 'X');
            board.print_board();


            println!(
                "result of check diag {}",
                board.check_diagonals('X')
            );


            println!(
                "result of check verts/horiz {}",
                board.check_vert_horiz('X')
            );
        },

        _ => {
            println!("Help");
        }
    }

}


