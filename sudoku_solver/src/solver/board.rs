use solver::cell::Cell;

pub struct Board {
    game_cells : [ [ Cell; 9]; 9]
}

impl Board {
    pub fn is_row_valid(&self, cells : &Vec<Cell>) -> bool {
        let mut vec = Vec::new();
        for cell in cells.iter() {
            if vec.contains(&cell) {
                return false;
            }
            vec.push(cell);
        }
        return true;
    }
}

impl Default for Board {
    fn default() -> Board {
        let mut grid: [ [ Cell; 9]; 9] = Default::default();

        for col in 0..9 {
            for row in 0..9 {
                grid[col][row] = Cell {current_value : 1, possible_values : vec![1]};
            }
        }
        return Board {game_cells : grid };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solver::cell::Cell;

    #[test]
    fn single_cell_contains_no_duplicates() {
        let board = Board {..Default::default()};
        let cell: Cell = Cell { current_value: 1, possible_values: vec![1] };
        let cells = vec![cell];
        assert_eq!(true, board.is_row_valid(&cells));
    }

    #[test]
    fn multiple_cells_contains_duplicates() {
        let board = Board {..Default::default()};
        let cell1: Cell = Cell { current_value: 1, possible_values: vec![1] };
        let cell2: Cell = Cell { current_value: 1, possible_values: vec![1] };

        let mut cells = Vec::new();
        cells.push(cell1);
        cells.push(cell2);

        assert_eq!(false, board.is_row_valid(&cells));
    }

    #[test]
    fn multiple_cells_contains_no_duplicates() {
        let board = Board {..Default::default()};
        let cell1: Cell = Cell { current_value: 1, possible_values: vec![1] };
        let cell2: Cell = Cell { current_value: 2, possible_values: vec![2] };
        let cell3: Cell = Cell { current_value: 3, possible_values: vec![3] };

        let mut cells = Vec::new();
        cells.push(cell1);
        cells.push(cell2);
        cells.push(cell3);

        assert_eq!(true, board.is_row_valid(&cells));
    }
}
