use array2d::{Array2D, Error};
use rand::Rng;
use std::{boxed::Box, sync::RwLock};
pub type Matrix<T> = Box<Array2D<T>>;
pub type TSMatrix<T> = RwLock<Box<Array2D<T>>>;
pub fn mem(dim: usize) -> Matrix<f64> {
    let arr = Array2D::filled_with(0.0, dim, dim);
    let matrix: Matrix<f64> = Box::new(arr);
    return matrix;
}
pub fn mem_lock(dim: usize) -> TSMatrix<f64> {
    let arr = Array2D::filled_with(0.0, dim, dim);
    let ts_matrix: TSMatrix<f64> = RwLock::new(Box::new(arr));
    return ts_matrix;
}
pub fn rand_seq(letter_vec: &mut Vec<char>, n: usize) {
    let mut rng = rand::rng();
    let mut tmp;
    for _ in 0usize..n {
        tmp = rng.random_range(0..4);
        match tmp {
            0 => {
                letter_vec.push('A');
            }
            1 => {
                letter_vec.push('G');
            }
            2 => {
                letter_vec.push('C');
            }
            3 => {
                letter_vec.push('U');
            }
            _ => {
                unreachable!();
            }
        }
    }
}
pub fn rna_array_init(matrix: &mut Matrix<f64>, n: usize, def: f64, def2: f64) {
    for i in 0..n + 5 {
        for j in 0..n + 5 {
            if i == j || i == 0 {
                matrix.set(i, j, def).unwrap();
            } else {
                matrix.set(i, j, def2).unwrap();
            }
        }
    }
}
pub fn rna_array_init_lock(matrix: &mut TSMatrix<f64>, n: usize, def: f64, def2: f64) {
    for i in 0..=n + 5 {
        for j in 0..=n + 5 {
            if i == j || i == 0 {
                matrix.write().unwrap().set(i, j, def).unwrap();
            } else {
                matrix.write().unwrap().set(i, j, def2).unwrap();
            }
        }
    }
}
