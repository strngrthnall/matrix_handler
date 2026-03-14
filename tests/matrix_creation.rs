#[cfg(test)]
mod matrix_creation{
    use matrix_handler::Matrix;

    #[test]
    fn create_good_matrix() {
        assert!(
            Matrix::new(
                3,
                3,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
            ).is_ok()
        );
    }

    #[test]
    fn matrix_mismatch_lines() {
        assert!(
            Matrix::new(
                2,
                3,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
            ).is_err()
        );
    }

    #[test]
    fn matrix_mismatch_columns() {
        assert!(
            Matrix::new(
                3,
                4,
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
            ).is_err()
        );
    }

}
