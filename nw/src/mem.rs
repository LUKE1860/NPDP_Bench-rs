use array2d::{Array2D, Error};
use rand::Rng;
use std::cmp::{max, min};
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
pub fn s(x: char, z: char) -> i32 {
    if x == z {
        return 1;
    }
    return -1;
}
fn nw_seq(
    m1: &mut Matrix<i32>,
    m2: &mut Matrix<i32>,
    f: &mut Matrix<i32>,
    w: Vec<i32>,
    a: Vec<char>,
    b: Vec<char>,
    n: usize,
) {
    rayon::scope(|x| {
        x.spawn(|_| {
            for i in 1..=n {
                for j in 1..=n {
                    m1[(i, j)] = i32::MIN;
                    for k in 1..=i {
                        m1[(i, j)] = max(m1[(i, j)], f[(i - k, j)] - w[k]);
                    }
                    m2[(i, j)] = i32::MIN;
                    for k in 1..=j {
                        m2[(i, j)] = max(m2[(i, j)], f[(i, j - k)] - w[k]);
                    }
                    f[(i, j)] = max(
                        0,
                        max(
                            f[(i - 1, j - 1)] + s(a[i], b[i]),
                            max(m1[(i, j)], m2[(i, j)]),
                        ),
                    );
                }
            }
        })
    });
}
