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
//! - **Indexação mutável**: altere elementos com `matrix[(linha, coluna)] = valor`.
//! - **Adição**: `&a + &b` (operador) ou `a.try_add(&b)` (com validação de dimensões).
//! - **Subtração**: `&a - &b` (operador) ou `a.try_sub(&b)` (com validação de dimensões).
//! - **Operações in-place**: `a += &b` e `a -= &b` sem alocação extra.
//!
//! ## Roadmap
//!
//! - Multiplicação escalar e matricial
//! - Transposição
//! - Iteradores sobre linhas e colunas
//! - `Display` formatado

use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

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


impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    /// Retorna uma referência mutável ao elemento na posição `(linha, coluna)`.
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
    /// let mut m = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
    /// m[(0, 0)] = 80;
    /// 
    /// assert_eq!(m[(0, 0)], 80);
    /// ```
    fn index_mut(&mut self, val_pos: (usize, usize)) -> &mut T {
        let (line, column) = val_pos;

        assert!(line < self.lines);
        assert!(column < self.columns);

        let matrix_columns = self.columns;

        let vec_index = line * matrix_columns + column;
        
        &mut self.values[vec_index]

    }
}


impl<T> Matrix<T> where T: Copy {
    /// Aplica uma operação elemento a elemento entre `self` e `rhs`,
    /// retornando uma nova matriz com os resultados.
    ///
    /// # Erros
    ///
    /// Retorna [`MatrixError::DimensionMismatch`] se as matrizes possuírem
    /// dimensões diferentes.
    fn try_elementwise_op<F>(
        &self, rhs: &Matrix<T>, operation: F
    ) -> Result<Matrix<T>, MatrixError> where 
        F: Fn(T, T) -> T {
        
        if self.lines != rhs.lines || self.columns != self.columns {
            return Err(MatrixError::DimensionMismatch);
        }

        let new_values: Vec<T> = self.values.iter()
            .zip(rhs.values.iter())
            .map(|(&val_a, &val_b)| operation(val_a, val_b))
            .collect();

        Ok(Matrix { 
            lines: self.lines, 
            columns: self.columns, 
            values: new_values 
        })
    }
}

/// Soma elemento a elemento de duas matrizes via operador `+`.
///
/// Permite a sintaxe `&a + &b`, onde ambas as referências são consumidas
/// e uma nova [`Matrix<T>`] é produzida.
///
/// # Panics
///
/// Esta implementação **não** valida dimensões. Se as matrizes tiverem
/// tamanhos diferentes, o comportamento depende de [`Iterator::zip`]
/// (para silenciosamente). Prefira [`MatrixMath::try_add`] para validação segura.
///
/// # Exemplos
///
/// ```rust
/// use matrix_handler::Matrix;
///
/// let a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
/// let b = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
///
/// let c = &a + &b;
/// assert_eq!(c[(0, 0)], 11);
/// assert_eq!(c[(1, 1)], 44);
/// ```
impl<T> Add<&Matrix<T>> for &Matrix<T> where T: Add<Output = T> + Copy {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Matrix<T> {
        let sum = self.values
            .iter()
            .zip(rhs.values.iter())
            .map(|(val_a, val_b)| *val_a + *val_b)
            .collect();

        Matrix { 
            lines: self.lines, 
            columns: self.columns, 
            values: sum 
        }
    }
}

/// Soma in-place via operador `+=`.
///
/// Modifica `self` diretamente somando os elementos de `rhs`,
/// evitando alocação de uma nova matriz.
///
/// # Exemplos
///
/// ```rust
/// use matrix_handler::Matrix;
///
/// let mut a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
/// let b = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
///
/// a += &b;
/// assert_eq!(a[(0, 0)], 11);
/// assert_eq!(a[(1, 1)], 44);
/// ```
impl<T> AddAssign<&Matrix<T>> for Matrix<T> where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: &Matrix<T>) {
        self.values
            .iter_mut()
            .zip(rhs.values.iter())
            .for_each(|(val_a, val_b)| *val_a += *val_b);
    
    }
}

/// Subtração elemento a elemento de duas matrizes via operador `-`.
///
/// Permite a sintaxe `&a - &b`, produzindo uma nova [`Matrix<T>`].
///
/// # Panics
///
/// Não valida dimensões. Prefira [`MatrixMath::try_sub`] para validação segura.
///
/// # Exemplos
///
/// ```rust
/// use matrix_handler::Matrix;
///
/// let a = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
/// let b = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
///
/// let c = &a - &b;
/// assert_eq!(c[(0, 0)], 9);
/// assert_eq!(c[(1, 1)], 36);
/// ```
impl<T> Sub<&Matrix<T>> for &Matrix<T> where T: Sub<Output = T> + Copy {
    type Output = Matrix<T>;

    fn sub(self, rhs: &Matrix<T>) -> Matrix<T> {
        let sum = self.values
            .iter()
            .zip(rhs.values.iter())
            .map(|(val_a, val_b)| *val_a - *val_b)
            .collect();

        Matrix { 
            lines: self.lines, 
            columns: self.columns, 
            values: sum 
        }
    }
}

/// Subtração in-place via operador `-=`.
///
/// Modifica `self` diretamente subtraindo os elementos de `rhs`,
/// evitando alocação de uma nova matriz.
///
/// # Exemplos
///
/// ```rust
/// use matrix_handler::Matrix;
///
/// let mut a = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
/// let b = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
///
/// a -= &b;
/// assert_eq!(a[(0, 0)], 9);
/// assert_eq!(a[(1, 1)], 36);
/// ```
impl<T> SubAssign<&Matrix<T>> for Matrix<T> where T: SubAssign<T> + Copy {
    fn sub_assign(&mut self, rhs: &Matrix<T>) {
        self.values
            .iter_mut()
            .zip(rhs.values.iter())
            .for_each(|(val_a, val_b)| *val_a -= *val_b);
    }
}

/// Trait para operações matemáticas com validação de dimensões.
///
/// Diferente dos operadores (`+`, `-`, `+=`, `-=`), os métodos dessa trait
/// verificam se as dimensões das matrizes são compatíveis **antes** de operar,
/// retornando `Result` em vez de entrar em pânico ou produzir resultados truncados.
///
/// # Exemplos
///
/// ```rust
/// use matrix_handler::{Matrix, MatrixMath, MatrixError};
///
/// let a = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();
/// let b = Matrix::new(2, 2, vec![10, 20, 30, 40]).unwrap();
///
/// // Adição segura
/// let soma = a.try_add(&b).unwrap();
/// assert_eq!(soma[(0, 0)], 11);
///
/// // Dimensões incompatíveis retornam erro
/// let c = Matrix::new(3, 3, vec![0; 9]).unwrap();
/// assert_eq!(b.try_add(&c), Err(MatrixError::DimensionMismatch));
/// ```
pub trait MatrixMath<Rhs = Self> {
    /// O tipo resultante da operação.
    type Output;
    /// O tipo de erro retornado em caso de falha.
    type Error;

    /// Soma duas matrizes com validação prévia de dimensões.
    ///
    /// # Erros
    ///
    /// Retorna [`MatrixError::DimensionMismatch`] se as matrizes possuírem
    /// dimensões diferentes.
    fn try_add(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;

    /// Subtrai duas matrizes com validação prévia de dimensões.
    ///
    /// # Erros
    ///
    /// Retorna [`MatrixError::DimensionMismatch`] se as matrizes possuírem
    /// dimensões diferentes.
    fn try_sub(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
    
}

/// Implementação de [`MatrixMath`] para referências de [`Matrix<T>`].
///
/// Requer que `T` implemente `Add`, `Sub` e `Copy`.
impl<T> MatrixMath<&Matrix<T>> for &Matrix<T> 
where 
    T: Add<Output = T> + Sub<Output = T> + Copy 
{
    type Output = Matrix<T>;
    type Error = MatrixError;
    
    fn try_add(self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        self.try_elementwise_op(rhs, |a, b| a + b)
    }

    fn try_sub(self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        self.try_elementwise_op(rhs, |a, b| a - b)
    }
}