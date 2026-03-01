use crate::mem::TSMatrix;
use std::cmp::{max, min};

fn zuker_seq(
    v: &mut TSMatrix<i32>,
    efl: &mut TSMatrix<i32>,
    w: &mut TSMatrix<i32>,
    ehf: &mut TSMatrix<i32>,
    n: usize,
) {
    rayon::scope(|x| {
        x.spawn(|_| {
            for i in n - 1..=0 {
                for j in i + 1..n {
                    for k in i + 1..j {
                        for m in k + 1..j {
                            if k - i + j - m > 2 && k - i + j - m < 30 {
                                v.write().unwrap()[(i, j)] = min(
                                    v.read().unwrap()[(k, m)] + efl.read().unwrap()[(i, j)],
                                    v.read().unwrap()[(i, j)],
                                );
                            }
                            w.write().unwrap()[(i, j)] += min(
                                min(w.read().unwrap()[(i, k)], w.read().unwrap()[(k + 1, j)]),
                                w.read().unwrap()[(i, j)],
                            );
                            if k < j - 1 {
                                v.write().unwrap()[(i, j)] = min(
                                    w.read().unwrap()[(i + 1, k)]
                                        + w.read().unwrap()[(k + 1, j - 1)],
                                    v.read().unwrap()[(i, j)],
                                )
                            }
                            v.write().unwrap()[(i, j)] = min(
                                min(
                                    v.read().unwrap()[(i + 1, j - 1)],
                                    ehf.read().unwrap()[(i, j)],
                                ),
                                v.read().unwrap()[(i, j)],
                            );
                            w.write().unwrap()[(i, j)] = min(
                                min(
                                    min(
                                        w.read().unwrap()[(i + 1, j)],
                                        w.read().unwrap()[(i, j - 1)],
                                    ),
                                    v.read().unwrap()[(i, j)],
                                ),
                                w.read().unwrap()[(i, j)],
                            )
                        }
                    }
                }
            }
        });
    });
}
fn zuker_pluto(
    v: &mut TSMatrix<i32>,
    efl: &mut TSMatrix<i32>,
    w: &mut TSMatrix<i32>,
    ehf: &mut TSMatrix<i32>,
    n: usize,
) {
}
