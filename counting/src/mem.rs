use array2d::{Array2D, Error};
use rand::Rng;
use std::{boxed::Box, sync::RwLock};
pub type Matrix<T> = Box<Array2D<T>>;
pub type TSMatrix<T> = RwLock<Box<Array2D<T>>>;
pub fn mem(dim: usize) -> Matrix<i32> {
    let arr = Array2D::filled_with(0, dim, dim);
    let matrix: Matrix<i32> = Box::new(arr);
    return matrix;
}
pub fn mem_lock(dim: usize) -> TSMatrix<i32> {
    let arr = Array2D::filled_with(0, dim, dim);
    let ts_matrix: TSMatrix<i32> = RwLock::new(Box::new(arr));
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
