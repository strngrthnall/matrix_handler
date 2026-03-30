#[cfg(test)]
mod matrix_math{
    use matrix_handler::{Matrix, MatrixMath};

    #[test]
    fn matrix_add() {
        let m1 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        let m2 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        let sum = m1.try_add(&m2).unwrap();

        let res: Matrix<i32> = Matrix::new(
            2, 2, vec![2, 4, 6, 8],
        ).unwrap();

        assert_eq!(sum, res)
    }

    #[test]
    fn matrix_sub() {
        let m1 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        let m2 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        let sub = m1.try_sub(&m2).unwrap();

        let res: Matrix<i32> = Matrix::new(
            2, 2, vec![0, 0, 0, 0],
        ).unwrap();

        assert_eq!(sub, res)
    }
    
    #[test]
    fn matrix_add_assign() {
        let mut m1 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        let m2 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        m1 += &m2;

        let res: Matrix<i32> = Matrix::new(
            2, 2, vec![2, 4, 6, 8],
        ).unwrap();

        assert_eq!(m1, res)
    }

    #[test]
    fn matrix_sub_assign() {
        let mut m1 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        let m2 = Matrix::new(
            2, 2, vec![1, 2, 3, 4],
        ).unwrap();

        m1 -= &m2;

        let res: Matrix<i32> = Matrix::new(
            2, 2, vec![0, 0, 0, 0],
        ).unwrap();

        assert_eq!(m1, res)
    }

    #[test]
    fn matrix_multiplication() {
        let m1 = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();
        let m2 = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();

        let m3 = m1 * &m2;

        assert_eq!(m3.values, [7, 10, 15, 22])
    }

    #[test]
    fn matrix_scalar_multiplication() {
        let m1 = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();
       

       let m2 = &m1 * 3;

        assert_eq!(m2.values, [3, 6, 9, 12])
    }

    #[test]
    fn matrix_multiplication_assign() {
        let mut m1 = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();
        let m2 = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();

        m1 *= &m2;

        assert_eq!(m1.values, [7, 10, 15, 22])
    }

    #[test]
    fn matrix_scalar_multiplication_assign() {
        let mut m1 = Matrix::new(2, 2, vec![1,2,3,4]).unwrap();
       

        m1 *= 3;

        assert_eq!(m1.values, [3, 6, 9, 12])
    }

    #[test]
    fn matrix_division() {
        let m1 = Matrix::new(2, 2, vec![3, 6, 9, 12]).unwrap();

        let m3 = &m1 / 3;

        assert_eq!(m3.values, [1, 2, 3, 4])
    }

    #[test]
    fn matrix_division_assign() {
        let mut m1 = Matrix::new(2, 2, vec![3, 6, 9, 12]).unwrap();

        m1 /= 3;

        assert_eq!(m1.values, [1, 2, 3, 4])
    }


    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn bad_matrix_multiplication() {
        let m1 = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
        let m2 = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();

        let m3 = m1 * &m2;

        assert_eq!(m3.values, [7, 10, 15, 22])
    }

}