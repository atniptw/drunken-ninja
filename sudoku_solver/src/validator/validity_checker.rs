pub fn is_valid(cells : &Vec<i32>) -> bool {
    let mut vec = Vec::new();
    for cell in cells.iter() {
        if vec.contains(&cell) {
            return false;
        }
        vec.push(cell);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_cell_contains_no_duplicates() {
        let cell = vec![1];
        assert_eq!(true, is_valid(&cell));
    }

    #[test]
    fn multiple_cells_contains_duplicates() {
        let cell = vec![1, 1];
        assert_eq!(false, is_valid(&cell));
    }

    #[test]
    fn multiple_cells_contains_no_duplicates() {
        let cell = vec![1, 2, 3];
        assert_eq!(true, is_valid(&cell));
    }

}
