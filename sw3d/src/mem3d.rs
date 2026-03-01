use ndarray::{Array3, Array4};
use rand::Rng;
use std::cmp::{max, min};
use std::{boxed::Box, sync::RwLock};
pub type TSMatrix3d<T> = RwLock<Box<Array3<T>>>;
pub type TSMatrix4d<T> = RwLock<Box<Array4<T>>>;
pub fn mem_lock_3d(dim: usize) -> TSMatrix3d<i32> {
    let arr = Array3::from_elem((dim, dim, dim), 0);
    let ts_matrix: TSMatrix3d<i32> = RwLock::new(Box::new(arr));
    return ts_matrix;
}
pub fn mem_lock_4d(dim: usize) -> TSMatrix4d<i32> {
    let arr = Array4::from_elem((dim, dim, dim, dim), 0);
    let ts_matrix: TSMatrix4d<i32> = RwLock::new(Box::new(arr));
    return ts_matrix;
}
pub fn rand_seq(letter_vec: &mut Vec<char>, n: usize) {
    let mut rng = rand::rng();
    let mut tmp;
    for _ in 0usize..=n {
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
