pub struct Cell {
    pub current_value : i32,
    pub possible_values : Vec<i32>
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        return self.current_value == other.current_value &&
        self.possible_values == other.possible_values;
    }
}

impl Default for Cell {
    fn default() -> Cell {
        return Cell {current_value : -1, possible_values : vec![1,2,3,4,5,6,7,8,9]};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_are_equal() {
        let cell1: Cell = Cell { current_value: 1, possible_values: vec![1] };
        let cell2: Cell = Cell { current_value: 1, possible_values: vec![1] };

        assert_eq!(true, cell1.eq(&cell2));
    }

    #[test]
    fn cells_are_not_equal() {
        let cell1: Cell = Cell { current_value: 1, possible_values: vec![1] };
        let cell2: Cell = Cell { current_value: 2, possible_values: vec![2] };

        assert_eq!(false, cell1.eq(&cell2));
    }
}
