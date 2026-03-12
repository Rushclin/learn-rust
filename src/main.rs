use std::fmt;

fn reverse(pair: (i32, String)) -> (String, i32) {
    let (int_params, str_params) = pair;

    return (str_params, int_params);
}

fn matrix_reverse(matrix: Matrix) -> Matrix{
    let Matrix(a, b, c, d) = matrix;
    Matrix(d, c, b, a)
}

fn matrix_transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;
    Matrix(a, c, b, d)
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Matrix(i32, i32, i32, i32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }   
}

fn main() {
    let long_tule = (10, String::from("Takam"), true);
    let matrix = Matrix(1, 2, 3, 4);

    println!("Hello, world!");
    println!(
        "I want to reverse (10, 'Takam') => {:?}",
        reverse((10, String::from("Takam")))
    );
    println!("Long tuple: {:?}", long_tule);
    println!("Ma Matrix: {}", matrix);
    println!("Ma Matrix transpose: {}", matrix_transpose(matrix));
    println!("Ma Matrix reverse: {}", matrix_reverse(matrix));
}
