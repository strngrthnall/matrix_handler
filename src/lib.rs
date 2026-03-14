//! # Matrix Handler
//!
//! Uma crate para criação e manipulação de matrizes genéricas em Rust.
//!
//! `matrix_handler` oferece uma estrutura de matriz eficiente baseada em um
//! vetor unidimensional contíguo na memória (*flat array*), garantindo boa
//! localidade de cache e performance superior a abordagens com vetores aninhados.
//!
//! ## Início rápido
//!
//! ```rust
//! use matrix_handler::Matrix;
//!
//! // Cria uma matriz 3×3
//! let matrix = Matrix::new(3, 3, vec![
//!     1, 2, 3,
//!     4, 5, 6,
//!     7, 8, 9,
//! ]).expect("dimensões devem corresponder ao tamanho do vetor");
//!
//! // Acessa o elemento na linha 1, coluna 2 (indexação começa em 0)
//! assert_eq!(matrix[(1, 2)], 6);
//! ```
//!
//! ## Funcionalidades
//!
//! - **Genérica**: funciona com qualquer tipo `T`.
//! - **Validação de dimensões**: retorna erro se o vetor de valores não
//!   corresponder às dimensões informadas.
//! - **Indexação ergonômica**: acesse elementos com a sintaxe `matrix[(linha, coluna)]`.
//!
//! ## Roadmap
//!
//! - Adição de matrizes (`Add` / `AddAssign`)
//! - Subtração de matrizes
//! - Multiplicação escalar e matricial
//! - Transposição
//! - Iteradores sobre linhas e colunas
//! - `Display` formatado

use std::ops::Index;

/// Erros que podem ocorrer durante operações com matrizes.
///
/// # Variantes
///
/// - [`DimensionMismatch`](MatrixError::DimensionMismatch) — as dimensões
///   informadas não correspondem ao número de elementos fornecidos.
#[derive(Debug, PartialEq)]
pub enum MatrixError {
    /// O número de elementos no vetor de valores não é igual a `linhas × colunas`.
    ///
    /// # Exemplo
    ///
    /// ```rust
    /// use matrix_handler::{Matrix, MatrixError};
    ///
    /// let result = Matrix::new(2, 2, vec![1, 2, 3]);
    /// assert_eq!(result, Err(MatrixError::DimensionMismatch));
    /// ```
    DimensionMismatch,
}

/// Uma matriz genérica de dimensões `linhas × colunas`.
///
/// Os elementos são armazenados internamente em um vetor contíguo
/// (*row-major order*), onde o índice linear é calculado por:
///
/// ```text
/// índice = linha * colunas + coluna
/// ```
///
/// # Exemplos
///
/// ```rust
/// use matrix_handler::Matrix;
///
/// // Matriz 2×3 de inteiros
/// let m = Matrix::new(2, 3, vec![
///     10, 20, 30,
///     40, 50, 60,
/// ]).unwrap();
///
/// assert_eq!(m[(0, 0)], 10);
/// assert_eq!(m[(1, 2)], 60);
/// ```
///
/// ```rust
/// use matrix_handler::Matrix;
///
/// // Também funciona com floats
/// let m = Matrix::new(1, 3, vec![1.0, 2.5, 3.7]).unwrap();
/// assert_eq!(m[(0, 1)], 2.5);
/// ```
#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    lines: usize,
    columns: usize,
    values: Vec<T>,
}

impl<T> Matrix<T> {
    /// Cria uma nova matriz com as dimensões e valores informados.
    ///
    /// # Parâmetros
    ///
    /// - `lines` — número de linhas da matriz.
    /// - `columns` — número de colunas da matriz.
    /// - `values` — vetor com os elementos em *row-major order*.
    ///
    /// # Erros
    ///
    /// Retorna [`MatrixError::DimensionMismatch`] se
    /// `values.len() != lines * columns`.
    ///
    /// # Exemplos
    ///
    /// ```rust
    /// use matrix_handler::Matrix;
    ///
    /// // Criação bem-sucedida
    /// let matrix = Matrix::new(2, 2, vec![1, 2, 3, 4]);
    /// assert!(matrix.is_ok());
    ///
    /// // Falha: 3×3 precisa de 9 elementos, mas só 4 foram fornecidos
    /// let bad = Matrix::new(3, 3, vec![1, 2, 3, 4]);
    /// assert!(bad.is_err());
    /// ```
    pub fn new(lines: usize, columns: usize, values: Vec<T>) -> Result<Matrix<T>, MatrixError> {
        let size = lines * columns;

        if size != values.len() {
            return Err(MatrixError::DimensionMismatch)
        }
        
        Ok(
            Matrix {
                lines,
                columns,
                values
            }
        )
    }

}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    /// Retorna uma referência ao elemento na posição `(linha, coluna)`.
    ///
    /// # Panics
    ///
    /// Entra em pânico se `linha >= self.lines` ou `coluna >= self.columns`.
    ///
    /// # Exemplos
    ///
    /// ```rust
    /// use matrix_handler::Matrix;
    ///
    /// let m = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
    /// assert_eq!(m[(0, 0)], 10);
    /// assert_eq!(m[(1, 1)], 40);
    /// ```
    fn index(&self, val_pos: (usize, usize)) -> &T {
        let (line, column) = val_pos;

        assert!(line < self.lines);
        assert!(column < self.columns);

        let matrix_columns = self.columns;

        let vec_index = line * matrix_columns + column;
        
        &self.values[vec_index]

    }
}
