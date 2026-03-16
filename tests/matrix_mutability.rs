#[cfg(test)]
mod matrix_mutability {
    use matrix_handler::Matrix;


    #[test]
    fn good_mutability() {
        let mut matrix:Matrix<i32> = Matrix::new(
            2, 
            2, 
            vec![2, 3, 1, 4],
        ).unwrap();

        matrix[(1,1)] = 16;

        assert_eq!(matrix[(1, 1)], 16)
    }


    #[test]
    #[should_panic(expected = "assertion failed")]
    fn off_bounds_mutability() {
        let mut matrix:Matrix<i32> = Matrix::new(
            2, 
            2, 
            vec![2, 3, 1, 4],
        ).unwrap();

        matrix[(1,55)] = 16;
    }
}