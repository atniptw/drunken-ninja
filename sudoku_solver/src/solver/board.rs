use solver::cell::Cell;

#[derive(Debug)]
pub struct Board {
    game_cells : [ [ Cell; 9]; 9]
}

impl Board {
    fn is_row_valid(&self, cells : &[Cell; 9]) -> bool {
        let mut vec = Vec::new();
        for cell in cells.iter() {
            if cell.current_value != -1 && vec.contains(&cell) {
                return false;
            }
            vec.push(cell);
        }
        return true;
    }
    pub fn is_valid(&self)  -> bool{
        for row in 0..9 {
            if !self.is_row_valid(&self.game_cells[row]) {
                return false;
            }
        }

        let mut col_slice: [Cell; 9] = Default::default();
        for col in 0..9 {
            col_slice[col] =
            Cell { current_value: self.game_cells[col][0].current_value,
                   possible_values: self.game_cells[col][0].possible_values.clone() };
        }

        if !self.is_row_valid(&col_slice) {
            return false;
        }

        return true;
    }
}

impl Default for Board {
    fn default() -> Board {
        let mut grid: [ [ Cell; 9 ]; 9 ] = Default::default();

        for col in 0..9 {
            for row in 0..9 {
                grid[col][row] = Cell {current_value : 1, possible_values : vec![1]};
            }
        }
        return Board { game_cells : grid };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solver::cell::Cell;

    // #[test]
    // fn single_cell_contains_no_duplicates() {
        // let board = Board {..Default::default()};
        // let cell: Cell = Cell { current_value: 1, possible_values: vec![1] };
        // let cells = vec![cell];
        // assert_eq!(true, board.is_row_valid(&cells));
    // }

    // #[test]
        // let board = Board {..Default::default()};
        // fn multiple_cells_contains_duplicates() {
        // let cell1: Cell = Cell { current_value: 1, possible_values: vec![1] };
        // let cell2: Cell = Cell { current_value: 1, possible_values: vec![1] };
        //
        // let mut cells = Vec::new();
        // cells.push(cell1);
        // cells.push(cell2);
        //
        // assert_eq!(false, board.is_row_valid(&cells));
    // }

    // #[test]
    // fn multiple_cells_contains_no_duplicates() {
        // let board = Board {..Default::default()};
        // let cell1: Cell = Cell { current_value: 1, possible_values: vec![1] };
        // let cell2: Cell = Cell { current_value: 2, possible_values: vec![2] };
        // let cell3: Cell = Cell { current_value: 3, possible_values: vec![3] };
        //
        // let mut cells = Vec::new();
        // cells.push(cell1);
        // cells.push(cell2);
        // cells.push(cell3);
        //
        // assert_eq!(true, board.is_row_valid(&cells));
    // }

    // #[test]
    // fn multiple_cells_contains_no_duplicates_and_mulitple_empty() {
        // let board = Board {..Default::default()};
        // let cell1: Cell = Cell { current_value: 1, possible_values: vec![1] };
        // let cell2: Cell = Cell { current_value: -1, possible_values: vec![1,2,3,4,5,6,7,8,9] };
        // let cell3: Cell = Cell { current_value: -1, possible_values: vec![1,2,3,4,5,6,7,8,9] };
        //
        // let mut cells = Vec::new();
        // cells.push(cell1);
        // cells.push(cell2);
        // cells.push(cell3);
        //
        // assert_eq!(true, board.is_row_valid(&cells));
    // }

    #[test]
    fn board_contains_no_duplicates_one_row() {
        let mut grid: [ [ Cell; 9 ]; 9 ] = Default::default();

        for row in 0i32..9 {
            grid[0][row as usize] = Cell {current_value : row, possible_values : vec![row]};
        }

        let board = Board { game_cells : grid };
        assert_eq!(true, board.is_valid());
    }

    #[test]
    fn board_with_duplicates_one_row() {
        let mut grid: [ [ Cell; 9 ]; 9 ] = Default::default();

        for row in 0i32..9 {
            grid[0][row as usize] = Cell {current_value : 1, possible_values : vec![1]};
        }

        let board = Board { game_cells : grid };
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_contains_no_duplicates_two_rows() {
        let mut grid: [ [ Cell; 9 ]; 9 ] = Default::default();

        for row in 0i32..9 {
            grid[0][row as usize] = Cell {current_value : row, possible_values : vec![row]};
        }

        for row in 0i32..9 {
            grid[1][row as usize] = Cell {current_value : 9 - row, possible_values : vec![row]};
        }

        let board = Board { game_cells : grid };
        assert_eq!(true, board.is_valid());
    }

    #[test]
    fn board_with_duplicates_row_two() {
        let mut grid: [ [ Cell; 9 ]; 9 ] = Default::default();

        for row in 0i32..9 {
            grid[0][row as usize] = Cell {current_value : row, possible_values : vec![row]};
        }

        for row in 0i32..9 {
            grid[1][row as usize] = Cell {current_value : 1, possible_values : vec![1]};
        }

        let board = Board { game_cells : grid };
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_with_duplicates_col() {
        let mut grid: [ [ Cell; 9 ]; 9 ] = Default::default();

        for col in 0i32..9 {
            grid[col as usize][0] = Cell {current_value : 1, possible_values : vec![1]};
        }

        let board = Board { game_cells : grid };
        assert_eq!(false, board.is_valid());
    }

    #[test]
    fn board_contains_no_duplicates_col() {
        let mut grid: [ [ Cell; 9 ]; 9 ] = Default::default();

        for col in 0i32..9 {
            grid[col as usize][0] = Cell {current_value : col, possible_values : vec![col]};
        }

        let board = Board { game_cells : grid };
        assert_eq!(true, board.is_valid());
    }
}
