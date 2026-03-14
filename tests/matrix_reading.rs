#[cfg(test)]
mod matrix_reading {
    use matrix_handler::Matrix;

      #[test]
    fn index_find_good() {
        let matrix: Matrix<i32> = Matrix::new(
            3,
            3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        ).unwrap();

        let value = matrix[(0,0)];

        assert_eq!(value, 1)
    }

    #[test]
    fn index_find_wrong() {
        let matrix: Matrix<i32> = Matrix::new(
            3,
            3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        ).unwrap();

        let value = matrix[(0,2)];

        assert_ne!(value, 1)
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn index_find_mismatch_line() {
        let matrix: Matrix<i32> = Matrix::new(
            3,
            3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        ).unwrap();

        let _ = matrix[(9,0)];

    }


    #[test]
    #[should_panic(expected = "assertion failed")]
    fn index_find_mismatch_column() {
        let matrix: Matrix<i32> = Matrix::new(
            3,
            3,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        ).unwrap();

        let _ = matrix[(0,9)];

    }
}