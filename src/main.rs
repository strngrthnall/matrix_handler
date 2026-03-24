use matrix_handler::Matrix;

fn main() {
    let m = Matrix::new(2, 3, vec![0, 12, 300, 4, 56, 700]).unwrap();

    print!("{}", m);
}