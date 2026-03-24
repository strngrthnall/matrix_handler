#[cfg(test)]
mod matrix_display {
    use matrix_handler::Matrix;

    #[test]
    fn display_print() {
        let m = Matrix::new(2, 3, vec![1, 20, 300, 4, 50, 600]).unwrap();
        let output = format!("{}", m);

        let expected = "   1  20 300\n   4  50 600";
        assert_eq!(output, expected)
    }

}