use array2d::Array2D;
use rand::Rng;
use std::sync::RwLock;
//f128 is supported on nightly channel
pub type TSMatrix<T> = RwLock<Box<Array2D<T>>>;
pub fn mem(n: usize) -> TSMatrix<f128> {
    let arr: Array2D<f128> = Array2D::filled_with(0.0, n + 5, n + 5);
    let ts_matrix: TSMatrix<f128> = RwLock::new(Box::new(arr));
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
