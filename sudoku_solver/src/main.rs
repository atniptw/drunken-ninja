extern crate sudoku_solver;

use sudoku_solver::validator::validity_checker;

fn main() {
    let cell = vec![1];
    println!("{:?}", validity_checker::is_valid(&cell));
}
