use array2d::Array2D;
use std::cmp::{max, min};
use std::sync::RwLock;
pub type TSMatrix<T> = RwLock<Box<Array2D<T>>>;
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn ceild(f_num: f64, s_num: f64) -> usize {
    return f64::ceil(f_num / s_num) as usize;
}
fn can_pair(input: &Vec<char>, a: usize, b: usize) -> f128 {
    match (input[a], input[b]) {
        ('A', 'U') => return 1.0,
        ('U', 'A') => return 1.0,
        ('G', 'C') => return 1.0,
        ('C', 'G') => return 1.0,
        ('G', 'U') => return 1.0,
        ('U', 'G') => return 1.0,
        _ => return 0.0,
    }
}
pub fn oryg(s: TSMatrix<f128>, n: usize, rna: Vec<char>) {
    for i in n - 1..=0 {
        if i % 100 == 0 {
            for j in i + 1..n {
                for k in 0..j - i {
                    s.write().unwrap()[(i, j)] = f128::max(
                        s.write().unwrap()[(i, k + i)] + s.read().unwrap()[(k + i + 1, j)],
                        s.read().unwrap()[(i, j)],
                    );
                }
                for k in 0..1 {
                    s.write().unwrap()[(i, j)] = f128::max(
                        s.read().unwrap()[(i, j)],
                        s.read().unwrap()[(i + 1, j - 1)] + can_pair(&rna, i, j),
                    );
                }
            }
        }
    }
}
pub fn pluto(s: TSMatrix<f128>, n: usize, rna: Vec<char>) {
    if n >= 2 {
        for t2 in max(
            s.read().unwrap().row_len() - 1,
            ceild((-(n as i32) - 13) as f64, 16f64),
        )..=floord((n - 1) as f64, 16f64)
        {
            let lbp = max(0, t2);
            let ubp = min(
                floord((n - 1) as f64, 16f64),
                floord((16 * t2 + n + 13) as f64, 16f64),
            );
            rayon::broadcast(|_| {
                for t4 in lbp..=ubp {
                    for t5 in max(
                        max((s.read().unwrap().row_len() - n) + 2, 16 * t2 - 16 * t4),
                        (-16 * t4 as i32 - 14) as usize,
                    )..=min(0, 16 * t2 - 16 * t4 + 15)
                    {
                        for t7 in max(16 * t4, (s.read().unwrap().row_len() - t5) + 1)
                            ..=min(n - 1, 16 * t4 + 15)
                        {
                            for t9 in 0..=t5 + t7 - 1 {
                                s.write().unwrap()[((s.read().unwrap().row_len() - t5), t7)] =
                                    f128::max(
                                        s.read().unwrap()[(
                                            (s.read().unwrap().row_len() - t5),
                                            t9 + (s.read().unwrap().row_len() - t5),
                                        )] + s.read().unwrap()
                                            [(t9 + (s.read().unwrap().row_len() - t5), t7)],
                                        s.read().unwrap()[(s.read().unwrap().row_len() - t5, t7)],
                                    );
                            }
                            s.write().unwrap()[((s.read().unwrap().row_len() - t5), t7)] =
                                f128::max(
                                    s.read().unwrap()[((s.read().unwrap().row_len() - t5), t7)],
                                    s.read().unwrap()
                                        [((s.read().unwrap().row_len() - t5) + 1, t7 - 1)]
                                        + can_pair(&rna, s.read().unwrap().row_len() - t5, t7),
                                );
                        }
                    }
                }
            });
        }
    }
}
pub fn tilecorr(s: TSMatrix<f128>, n: usize, rna: Vec<char>) {
    for c1 in 1..n + floord((n - 2) as f64, 128f64) {
        rayon::broadcast(|_| {
            for c3 in max(0, (-(n as i32) + c1 as i32 + 1) as usize)..=(c1 - 1) / 129 {
                for c4 in 0..=1 {
                    if c4 == 1 {
                        for c9 in n - c1 + 129 * c3..min(n - 1, n - c1 + 129 * c3 + 127) {
                            for c10 in max(0, n - c1 + 129 * c3 - c9 + 1)..=1 {
                                if c10 == 1 {
                                    s.write().unwrap()[(n - c1 + c3 - 1, c9)] = f128::max(
                                        s.read().unwrap()[(n - c1 + c3 + 1, c9)],
                                        s.read().unwrap()[(n - c1 + c3 - 1, c9 - 1)]
                                            + can_pair(&rna, n - c1 + c3 - 1, c9),
                                    );
                                } else {
                                    for c11 in 128 * c3 + 1
                                        ..=(-(n as i32) + c1 as i32 - c3 as i32 + c9 as i32)
                                            as usize
                                    {
                                        s.write().unwrap()[(n - c1 + c3 - 1, c9)] = f128::max(
                                            s.read().unwrap()
                                                [(n - c1 + c3 + 1, c11 + (n - c1 + c3 + 1))]
                                                + s.read().unwrap()[(c11 + (n - c1 + c3 - 1), c9)],
                                            s.read().unwrap()[(n - c1 + c3 - 1, c9)],
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                        for c5 in 0..=8 * c3 {
                            for c9 in n - c1 + 129 * c3..=min(n - 1, n - c1 + 120 * c3 + 127) {
                                for c11 in 16 * c5..=min(128 * c3, 16 * c5 + 15) {
                                    s.write().unwrap()[(n - c1 + c3 - 1, c9)] = f128::max(
                                        s.read().unwrap()
                                            [(n - c1 + c3 + 1, c11 + (n - c1 + c3 - 1))]
                                            + s.read().unwrap()[(c11 + (n - c1 + c3 - 1) + 1, c9)],
                                        s.read().unwrap()[(n - c1 + c3 - 1, c9)],
                                    );
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
pub fn tstile(s: TSMatrix<f128>, n: usize, rna: Vec<char>) {
    for c0 in 0..floord((n - 2) as f64, 8f64) {
        rayon::broadcast(|_| {
            for c1 in (c0 + 1) / 2..=min(c0, (n - 1) / 16) {
                for c3 in
                    16 * c0 - 16 * c1 + 1..=min(min(n - 1, 16 * c1 + 15), 16 * c0 - 16 * c1 + 16)
                {
                    for c4 in 0..c0 - c1 {
                        for c6 in max(
                            (-(n as i32) + 16 * c1 as i32 + 1) as usize,
                            (-(n as i32) + c3 as i32 + 1) as usize,
                        )
                            ..min(0, (-(n as i32) + 16 * c1 as i32 + 16) as usize)
                        {
                            for c10 in 16 * c4..=min(c3 - 1, 16 * c4 + 15) {
                                s.write().unwrap()[(s.read().unwrap().row_len() - c6, c3 - c6)] =
                                    f128::max(
                                        s.read().unwrap()[(
                                            s.read().unwrap().row_len() - c6,
                                            c10 + (s.read().unwrap().row_len() - c6),
                                        )] + s.read().unwrap()[(
                                            c10 + (s.read().unwrap().row_len() - c6) + 1,
                                            c3 - c6,
                                        )],
                                        s.read().unwrap()
                                            [(s.read().unwrap().row_len() - c6, c3 - c6)],
                                    );
                            }
                            if c1 + c4 == c0 && 16 * c0 + c6 + 15 >= 16 * c1 + c3 {
                                s.write().unwrap()[(s.read().unwrap().row_len() - c6, c3 - c6)] =
                                    f128::max(
                                        s.read().unwrap()
                                            [(s.read().unwrap().row_len() - c6, c3 - c6)],
                                        s.read().unwrap()[(
                                            (s.read().unwrap().row_len() - c6) + 1,
                                            (c3 - c6) - 1,
                                        )] + can_pair(&rna, rna.len() - c6, c3 - c6),
                                    );
                            }
                        }
                        for c4 in max(
                            c0 - c1 + 1,
                            s.read().unwrap().row_len() - c1 + (n + c3) / 16 - 1,
                        )
                            ..=min(
                                (n - 1) / 16,
                                s.read().unwrap().row_len() - c1 + (n + c3 - 1) / 16,
                            )
                        {
                            for c6 in max(
                                max(
                                    -(n as i32) + 16 * c1 as i32 + 1,
                                    -(n as i32) + c3 as i32 + 1,
                                ) as usize,
                                c3 - 16 * c4 - 15,
                            )
                                ..=min((-(n as i32) + 16 * c1 as i32 + 16) as usize, c3 - 16 * c4)
                            {
                                s.write().unwrap()[(s.read().unwrap().row_len() - c6, c3 - c6)] =
                                    f128::max(
                                        s.read().unwrap()
                                            [(s.read().unwrap().row_len() - c6, c3 - c6)],
                                        s.read().unwrap()[(
                                            (s.read().unwrap().row_len() - c6) + 1,
                                            (c3 - c6) - 1,
                                        )] + can_pair(&rna, rna.len() - c6, c3 - c6),
                                    );
                            }
                        }
                    }
                }
            }
        });
    }
}
