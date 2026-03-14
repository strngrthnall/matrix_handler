<p align="center">
  <h1 align="center">🧮 Matrix Handler</h1>
  <p align="center">
    Uma crate Rust para criação e manipulação de matrizes genéricas com foco em performance e ergonomia.
  </p>
  <p align="center">
    <img src="https://img.shields.io/badge/rust-2024_edition-orange?logo=rust" alt="Rust Edition">
    <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version">
    <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
    <img src="https://img.shields.io/badge/status-em_desenvolvimento-yellow" alt="Status">
  </p>
</p>

---

## Sobre

**Matrix handler** é uma biblioteca Rust leve e genérica para trabalhar com matrizes. Os elementos são armazenados em um vetor contíguo na memória (*flat array* em *row-major order*), garantindo excelente localidade de cache — muito mais eficiente do que a abordagem clássica de "vetor de vetores".

```text
Matriz 3×3 lógica:          Armazenamento interno (Vec<T>):

┌───┬───┬───┐
│ 1 │ 2 │ 3 │               [1, 2, 3, 4, 5, 6, 7, 8, 9]
├───┼───┼───┤                        ↑
│ 4 │ 5 │ 6 │               índice = linha × colunas + coluna
├───┼───┼───┤
│ 7 │ 8 │ 9 │
└───┴───┴───┘
```

## Funcionalidades

| Funcionalidade | Status |
|---|---|
| Criação de matrizes genéricas (`Matrix<T>`) | ✅ Implementado |
| Validação de dimensões na construção | ✅ Implementado |
| Indexação por `(linha, coluna)` | ✅ Implementado |
| Adição de matrizes (`Add` / `AddAssign`) | 🔜 Em breve |
| Subtração de matrizes | 🔜 Em breve |
| Multiplicação escalar | 🔜 Em breve |
| Multiplicação matricial | 🔜 Em breve |
| Transposição | 🔜 Em breve |
| Iteradores (linhas / colunas) | 🔜 Em breve |
| `Display` formatado | 🔜 Em breve |

## Início Rápido

### Adicionando ao seu projeto

O projeto ainda vai ser carregado ao crates.io, por enquanto para utilizar deve-se baixar e inserir o arquivo ao seu projeto.

```toml
[dependencies]
matrix_handler = "0.1.0"
```

### Criando uma matriz

```rust
use matrix_handler::Matrix;

fn main() {
    let matrix = Matrix::new(3, 3, vec![
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    ]).expect("dimensões devem corresponder ao tamanho do vetor");

    // Acessa o elemento na linha 1, coluna 2 (indexação começa em 0)
    println!("Elemento [1][2] = {}", matrix[(1, 2)]); // → 6
}
```

### Tratamento de erros

```rust
use matrix_handler::{Matrix, MatrixError};

fn main() {
    // Erro: 2×2 exige 4 elementos, mas apenas 3 foram fornecidos
    let resultado = Matrix::new(2, 2, vec![1, 2, 3]);

    match resultado {
        Ok(m) => println!("Matriz criada: {:?}", m),
        Err(MatrixError::DimensionMismatch) => {
            eprintln!("Erro: dimensões não correspondem ao número de elementos!");
        }
    }
}
```

### Funciona com qualquer tipo

```rust
use matrix_handler::Matrix;

// Inteiros
let int_matrix = Matrix::new(2, 2, vec![1, 2, 3, 4]).unwrap();

// Floats
let float_matrix = Matrix::new(2, 2, vec![1.0, 2.5, 3.7, 4.2]).unwrap();

// Strings
let str_matrix = Matrix::new(1, 3, vec!["a", "b", "c"]).unwrap();
```

## Arquitetura

```text
matrix_handler/
├── Cargo.toml              # Metadados e dependências
├── README.md               # Este arquivo
├── CONTEXT                 # Notas de design e decisões arquiteturais
├── src/
│   └── lib.rs              # Código principal: Matrix<T>, MatrixError
└── tests/
    ├── matrix_creation.rs  # Testes de criação e validação
    └── matrix_reading.rs   # Testes de indexação e leitura
```

### Decisões de design

- **Flat array vs. Vec\<Vec\<T\>\>**: armazenamento contíguo em memória para melhor performance de cache.
- **Genérica sobre `T`**: sem restrição de trait na struct — traits são exigidos apenas nos métodos que precisam deles.
- **Erros explícitos**: `Result<Matrix<T>, MatrixError>` no construtor em vez de `panic!`.
- **Indexação via trait `Index`**: permite a sintaxe natural `matrix[(i, j)]`.

## Executando os testes

```bash
cargo test
```

Saída esperada:

```text
running 3 tests ... ok    # matrix_creation
running 4 tests ... ok    # matrix_reading
```

## Gerando a documentação

```bash
cargo doc --open
```

## Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE).

---

<p align="center">
  Feito com 🦀 e Rust
</p>
