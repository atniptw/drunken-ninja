extern crate sudoku_solver;

use sudoku_solver::validator::validity_checker;

#[test]
fn single_cell_contains_no_duplicates_integration() {
    let cell = vec![1];
    assert_eq!(true, validity_checker::is_valid(&cell));
}
