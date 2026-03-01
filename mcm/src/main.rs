//fix mcm
use array2d::{Array2D, Error};
use mem::Matrix;
use mem::TSMatrix;
use rand::prelude::*;
use rayon::ThreadPool;
use rayon::ThreadPoolBuildError;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::env;
use std::ptr::read;
use std::thread;
use std::time::{Duration, Instant};
mod mem;
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn ceild(f_num: f64, s_num: f64) -> usize {
    return f64::ceil(f_num / s_num) as usize;
}
fn minsq(s: &TSMatrix<i32>, a: i32, b: i32, i: usize, j: usize, k: i32) -> i32 {
    if a < b {
        s.write().unwrap()[(i, j)] = k;
        return a;
    } else {
        return b;
    }
}
fn matrix_chain_order(p: Vec<i32>, n: usize, kind: i32, dim: usize) -> i32 {
    let m = mem::mem_lock(dim);
    let mut s = mem::mem_lock(dim);
    let start = Instant::now();
    for i in 1..n {
        m.write().unwrap()[(i, i)] = 0;
    }
    if kind == -1 {
        for l in 2..n {
            for i in 1..n - l + 1 {
                let j = i + l - 1;
                m.write().unwrap()[(i, j)] = i32::MAX;
                for k in i..=j - 1 {
                    let q = m.read().unwrap()[(i, k)]
                        + m.read().unwrap()[(k + 1, j)]
                        + m.read().unwrap()[(i, k)]
                        + p[i - 1] * p[k] * p[j];
                    if (q < m.read().unwrap()[(i, j)]) {
                        m.write().unwrap()[(i, j)] = q;
                        s.write().unwrap()[(i, j)] = q;
                    }
                }
            }
        }
        if kind == 1 {
            for l in 2..n {
                for i in 1..n - l + 1 {
                    m.write().unwrap()[(i, i + l - 1)] = i32::MAX;
                    for k in i..i + l - 1 - 1 {
                        m.write().unwrap()[(i, i + l - 1)] = minsq(
                            &s,
                            m.read().unwrap()[(i, k)]
                                + m.read().unwrap()[(k + 1, i + l - 1)]
                                + p[i - 1] * p[k] * p[i + l - 1],
                            m.read().unwrap()[(i, i + l - 1)],
                            i,
                            i + l - 1,
                            k as i32,
                        );
                    }
                }
            }
        }
    }
    if kind == 2 {
        let lbp = 0;
        let ubp = floord((n - 1) as f64, 16f64);
        rayon::broadcast(|_| {
            for t2 in lbp..=ubp {
                for t3 in 0..=min(
                    floord((n - 2) as f64, 16f64),
                    floord((-16 * t2 as i32 + n as i32) as f64, 16f64),
                ) {
                    for t4 in max(1, 16 * t3 as i32)
                        ..=min(
                            min((n as i32) - 2, -16 * (t2 as i32) + (n as i32)),
                            16 * (t3 as i32) + 15,
                        )
                    {
                        let lbv = max(2, 16 * t2 as i32);
                        let ubv = min(16 * t2 as i32 + 15, -(t4 as i32) + n as i32);
                        for t5 in lbv..ubv {
                            m.write().unwrap()[(t4 as usize, t4 as usize + t5 as usize - 1)] =
                                i32::MAX;
                        }
                    }
                }
            }
        });
        for t1 in 0..=floord((n - 1) as f64, 8f64) {
            let lbp = ceild(t1 as f64, 2f64);
            let ubp = min(floord(n as f64, 16f64), t1);
            rayon::broadcast(|_| {
                for t2 in lbp..=ubp {
                    for t3 in max(2, 16 * t1 - 16 * t2)
                        ..=min(min(n - 1, 16 * t2 + 14), 16 * t1 - 16 * t2 + 15)
                    {
                        for t4 in max(16 * t2, t3 + 1)..min(n, 16 * t2 + 15) {
                            for t5 in -(t3 as i32) + t4 as i32..=t4 as i32 - 2 {
                                m.write().unwrap()[(
                                    (m.read().unwrap().row_len() - t3) + t4,
                                    (m.read().unwrap().row_len() - t3) + t4,
                                )] = minsq(
                                    &s,
                                    m.read().unwrap()
                                        [((m.read().unwrap().row_len() - t3) + t4, t5 as usize)]
                                        + m.read().unwrap()[((
                                            t5 as usize + 1,
                                            (m.read().unwrap().row_len() - t3) + t4 + t3 - 1,
                                        ))]
                                        + p[(p.len() - t3 + t4) - 1]
                                            * p[t5 as usize]
                                            * p[(p.len() - t3 + t4) + t3 - 1],
                                    m.read().unwrap()[(
                                        (m.read().unwrap().row_len() - t3) + t4,
                                        ((m.read().unwrap().row_len() - t3) + t4) + t3 - 1,
                                    )],
                                    (m.read().unwrap().row_len() - t3) + t4,
                                    ((m.read().unwrap().row_len() - t3) + t4) + t3 - 1,
                                    t5,
                                );
                            }
                        }
                    }
                }
            });
        }
    }
    if kind == 3 {
        let lbp = 0;
        let ubp = floord((n - 1) as f64, 16f64);
        rayon::broadcast(|_| {
            for t2 in lbp..=ubp {
                for t3 in 0..=min(
                    floord((n - 2) as f64, 16f64),
                    floord((-16 * t2 as i32 + n as i32) as f64, 16f64),
                ) {
                    for t4 in max(1, 16 * t3 as i32)
                        ..=min(
                            min(n as i32 - 2, -16 * t2 as i32 + n as i32),
                            16 * t3 as i32 + 15,
                        )
                    {
                        let lbv = max(2, 16 * t2);
                        let ubv = min(16 * t2 as i32 + 15, -(t4 as i32) + n as i32) as usize;
                        for t5 in lbv..ubv {
                            m.write().unwrap()[(t4 as usize, t4 as usize + t5 as usize - 1)] =
                                i32::MAX;
                        }
                    }
                }
            }
        });
        for c1 in 0..n - 2 {
            rayon::broadcast(|_| {
                for c3 in 0..=(n - c1 - 3) / 128 {
                    for c5 in 0..=c1 / 16 {
                        for c9 in 128 * c3 + 1..=min(n - c1 - 2, 128 * c3 + 128) {
                            for c11 in 16 * c5 + c9..=min(c1 + c9, 16 * c5 + c9 + 15) {
                                m.write().unwrap()[(c9, c9 + (c1 + 2) - 1)] = minsq(
                                    &s,
                                    m.read().unwrap()[(c9, c11)]
                                        + m.read().unwrap()[(c11 + 1, c9 + (c1 + 2) - 1)]
                                        + p[c9 - 1] * p[c11] * p[c9 + (c1 + 2) - 1],
                                    m.read().unwrap()[(c9, c9 + (c1 + 2) - 1)],
                                    c9,
                                    c9 + (c1 + 2) - 1,
                                    c11 as i32,
                                );
                            }
                        }
                    }
                }
            });
        }
        for c1 in 0..floord((n - 3) as f64, 16f64) {
            rayon::broadcast(|_| {
                for c3 in 0..=-(c1 as i32) + (n as i32 - 3) {
                    for c5 in 0..=c1 {
                        if c1 >= c5 + 1 {
                            for c9 in 16 * c1 as i32 + 16 * c3 as i32 + 3
                                ..=min(n as i32, 16 * c1 as i32 + 16 * c3 as i32 + 18)
                            {
                                for c11 in -16 * c1 as i32 + 16 * c5 as i32 + c9 as i32 - 2
                                    ..=-16 * c1 as i32 + 16 * c5 as i32 + c9 as i32 + 13
                                {
                                }
                            }
                        } else {
                            for c7 in 16 * c1 + 2..=min(16 * c1 + 17, n - c3 as usize - 1) {
                                for c9 in max(16 * c1 + 16 * c3 as usize + 3, c7 + 1)
                                    ..=min(n, 16 * c1 + 16 * c3 as usize + 18)
                                {
                                    if (c9 >= 16 * c3 as usize + c7 as usize + 1) {
                                        if c7 >= 16 * c1 + 3 {
                                            for c11 in -(c7 as i32) + c9 as i32
                                                ..16 * c1 as i32 - c7 as i32 + c9 as i32
                                            {
                                                m.write().unwrap()[(
                                                    (m.read().unwrap().row_len() - c7) + c9,
                                                    ((m.read().unwrap().row_len() - c7) + c9) + c7
                                                        - 1,
                                                )] = minsq(
                                                    &s,
                                                    m.read().unwrap()[(
                                                        (m.read().unwrap().row_len() - c7) + c9,
                                                        c11 as usize,
                                                    )] + m.read().unwrap()[(
                                                        c11 as usize + 1,
                                                        ((m.read().unwrap().row_len() - c7) + c9)
                                                            - 1,
                                                    )] + p[((p.len() - c7) + c9) + c7 - 1]
                                                        * p[c11 as usize]
                                                        * p[((p.len() - c7) + c9) + c7 - 1],
                                                    m.read().unwrap()[(
                                                        (m.read().unwrap().row_len() - c7) + c9,
                                                        ((m.read().unwrap().row_len() - c7) + c9)
                                                            + c7
                                                            - 1,
                                                    )],
                                                    m.read().unwrap().row_len() - c7 + c9,
                                                    (m.read().unwrap().row_len() - c7 + c9) + c7
                                                        - 1,
                                                    c11,
                                                );
                                            }
                                            for c11 in 16 * c1 - c7 + c9..c9 - 1 {
                                                m.write().unwrap()[(
                                                    (m.read().unwrap().row_len() - c7) + c9,
                                                    ((m.read().unwrap().row_len() - c7) + c9) + c7
                                                        - 1,
                                                )] = minsq(
                                                    &s,
                                                    m.read().unwrap()[(
                                                        (m.read().unwrap().row_len() - c7) + c9,
                                                        c11,
                                                    )] + m.read().unwrap()[(
                                                        c11 + 1,
                                                        ((m.read().unwrap().row_len() - c7) + c9)
                                                            - 1,
                                                    )] + p[((p.len() - c7) + c9) + c7 - 1]
                                                        * p[c11]
                                                        * p[((p.len() - c7) + c9) + c7 - 1],
                                                    m.read().unwrap()[(
                                                        (m.read().unwrap().row_len() - c7) + c9,
                                                        ((m.read().unwrap().row_len() - c7) + c9)
                                                            + c7
                                                            - 1,
                                                    )],
                                                    m.read().unwrap().row_len() - c7 + c9,
                                                    (m.read().unwrap().row_len() - c7 + c9) + c7
                                                        - 1,
                                                    c11 as i32,
                                                );
                                            }
                                        } else {
                                            for c11 in -(c7 as i32) + c9 as i32..c9 as i32 - 1 {
                                                m.write().unwrap()[(
                                                    (m.read().unwrap().row_len() - c7) + c9,
                                                    ((m.read().unwrap().row_len() - c7) + c9) + c7
                                                        - 1,
                                                )] = minsq(
                                                    &s,
                                                    m.read().unwrap()[(
                                                        (m.read().unwrap().row_len() - c7) + c9,
                                                        c11 as usize,
                                                    )] + m.read().unwrap()[(
                                                        c11 as usize + 1,
                                                        ((m.read().unwrap().row_len() - c7) + c9)
                                                            - 1,
                                                    )] + p[((p.len() - c7) + c9) + c7 - 1]
                                                        * p[c11 as usize]
                                                        * p[((p.len() - c7) + c9) + c7 - 1],
                                                    m.read().unwrap()[(
                                                        (m.read().unwrap().row_len() - c7) + c9,
                                                        ((m.read().unwrap().row_len() - c7) + c9)
                                                            + c7
                                                            - 1,
                                                    )],
                                                    m.read().unwrap().row_len() - c7 + c9,
                                                    (m.read().unwrap().row_len() - c7 + c9) + c7
                                                        - 1,
                                                    c11,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    }
    if kind == 3 {
        let lbp = 0;
        let ubp = floord((n - 1) as f64, 16f64);
        rayon::broadcast(|_| {
            for t2 in lbp..=ubp {
                for t3 in 0..=min(
                    floord((n - 2) as f64, 16f64),
                    floord((-16 * t2 as i32 + n as i32) as f64, 16f64),
                ) {
                    for t4 in max(1, 16 * t3 as i32)
                        ..=min(
                            min(n as i32 - 2, -16 * t2 as i32 + n as i32),
                            16 * t3 as i32 + 15,
                        )
                    {
                        let lbv = max(2, 16 * t2);
                        let ubv = min(16 * t2 as i32 + 15, -(t4 as i32) + n as i32) as usize;
                        for t5 in lbv..ubv {
                            m.write().unwrap()[(t4 as usize, t4 as usize + t5 as usize - 1)] =
                                i32::MAX;
                        }
                    }
                }
            }
        });
        for c0 in 0..floord((n - 3) as f64, 16f64) {
            rayon::broadcast(|_| {
                for c1 in 0..=c0 {
                    for c3 in 16 * c0 + 16 * c1 + 5
                        ..min(
                            min(2 * n - 16 * c0 + 16 * c1 - 1, n + 16 * c1 + 17),
                            16 * c0 + 16 * c1 + 50,
                        )
                    {
                        for c4 in max(
                            c0 as i32 - c1 as i32,
                            -2 * c1 as i32 + (c3 as i32 - 2) / 16 - 2,
                        )
                            ..min(
                                (n as i32 - 2) / 16,
                                -(c1 as i32)
                                    + (16 * c0 as i32 + 16 * c1 as i32 + c3 as i32 + 12) / 32,
                            )
                        {
                            for c6 in max(
                                max(
                                    max(16 * c1 as i32 + 2, -(n as i32) + c3 as i32),
                                    -8 * c4 as i32 + c3 as i32 / 2 - 7,
                                ),
                                -8 * c0 as i32 + 8 * c1 as i32 + (c3 as i32 + 1) / 2 - 8,
                            )
                                ..=min(
                                    min(16 * c1 as i32 + 17, c3 as i32 - 16 * c4 as i32 - 2),
                                    -8 * c0 as i32 + 8 * c1 as i32 + (c3 as i32 + 1) / 2 - 1,
                                )
                            {
                                for c10 in max(16 * c4 as i32, c3 as i32 - 2 * c6)
                                    ..=min(16 * c4 as i32 + 15, c3 as i32 - c6 - 2)
                                {
                                    m.write().unwrap()[(
                                        c3 - 2 * c6 as usize,
                                        (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                    )] = minsq(
                                        &s,
                                        m.read().unwrap()
                                            [(c3 as usize - 2 * c6 as usize, c10 as usize)]
                                            + m.read().unwrap()[(
                                                c10 as usize + 1,
                                                (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                            )]
                                            + p[(c3 - 2 * c6 as usize) - 1]
                                                * p[c10 as usize]
                                                * p[(c3 - 2 * c6 as usize) + c6 as usize - 1],
                                        m.read().unwrap()[(
                                            c3 - 2 * c6 as usize,
                                            (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                        )],
                                        c3 - 2 * c6 as usize,
                                        (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                        c10,
                                    );
                                }
                            }
                        }
                    }
                }
            });
            if 1 == 0 {
                for c0 in 0..=floord((n - 3) as f64, 16f64) {
                    rayon::broadcast(|_| {
                        for c1 in 0..=c0 {
                            for c3 in 16 * c1 + 2..=min(n - 16 * c0 + 16 * c1 - 1, 16 * c1 + 17) {
                                for c4 in c0 - c1..=min(c0 - c1 + 1, (n - c3) / 16) {
                                    if (c0 == 0 && c1 == 0 && c4 == 0) {
                                        for c6 in 2..(c3 + 1) / 2 {
                                            for c10 in c3 - 2 * c6..c3 - c6 - 1 {
                                                m.write().unwrap()
                                                    [(c3 - 2 * c6, (c3 - 2 * c6) + c6 - 1)] = minsq(
                                                    &s,
                                                    m.read().unwrap()[(c3 - 2 * c6, c10)]
                                                        + m.read().unwrap()
                                                            [(c10 + 1, (c3 - 2 * c6) + c6 - 1)]
                                                        + p[((c3 - 2 * c6) - 1)]
                                                            * p[c10]
                                                            * p[(c3 - 2 * c6) + c6 - 1],
                                                    m.read().unwrap()
                                                        [(c3 - 2 * c6, (c3 - 2 * c6) + c6 - 1)],
                                                    c3 - 2 * c6,
                                                    (c3 - 2 * c6) + c6,
                                                    c10 as i32,
                                                );
                                            }
                                        }
                                    } else {
                                        for c8 in max(16 * c0 - 16 * c1 + 1, 16 * c4)
                                            ..=min(
                                                min(16 * c0 - 16 * c1 + 16, n - c3),
                                                16 * c4 + 15,
                                            )
                                        {
                                            m.write().unwrap()[(c8, c8 + c3 + 1)] = i32::MAX;
                                        }
                                    }
                                }
                            }
                            for c3 in max(5, n)..=min(17, 2 * n - 1) {
                                for c6 in max(2, -(n as i32) + c3 as i32)..(c3 as i32 + 1) {
                                    for c10 in c3 - 2 * c6 as usize..c3 - c6 as usize - 1 {
                                        m.write().unwrap()[(
                                            c3 - 2 * c6 as usize,
                                            (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                        )] = minsq(
                                            &s,
                                            m.read().unwrap()[(c3 - 2 * c6 as usize, c10)]
                                                + m.read().unwrap()[(
                                                    c10 + 1,
                                                    (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                                )]
                                                + p[((c3 - 2 * c6 as usize) - 1)]
                                                    * p[c10]
                                                    * p[(c3 - 2 * c6 as usize) + c6 as usize - 1],
                                            m.read().unwrap()[(
                                                c3 - 2 * c6 as usize,
                                                (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                            )],
                                            c3 - 2 * c6 as usize,
                                            (c3 - 2 * c6 as usize) + c6 as usize,
                                            c10 as i32,
                                        );
                                    }
                                }
                            }
                            for c3 in max(16 * c0 + 16 * c1 + 5, 16 * c1 + 18)
                                ..=min(
                                    min(2 * n - 16 * c0 + 16 * c1 - 1, n + 16 * c1 + 17),
                                    16 * c0 + 16 * c1 + 50,
                                )
                            {
                                for c4 in max(
                                    c0 as i32 - c1 as i32,
                                    -2 * c1 as i32 + (c3 as i32 - 2) / 16 - 2,
                                )
                                    ..=min(
                                        (n as i32 - 2) / 16,
                                        -(c1 as i32)
                                            + (16 * c0 as i32 + 16 * c1 as i32 + c3 as i32 + 12)
                                                / 32,
                                    )
                                {
                                    for c6 in max(
                                        max(
                                            max(16 * c1 as i32 + 2, -(n as i32) + c3 as i32),
                                            -8 * c4 as i32 + c3 as i32 / 2 - 7,
                                        ),
                                        -8 * c0 as i32 + 8 * c1 as i32 + (c3 as i32 + 1) / 2 - 8,
                                    )
                                        ..min(
                                            min(
                                                16 * c1 as i32 + 17,
                                                c3 as i32 - 16 * c4 as i32 - 2,
                                            ),
                                            -8 * c0 as i32 + 8 * c1 as i32 + (c3 as i32 + 1) / 2
                                                - 1,
                                        )
                                    {
                                        for c10 in max(16 * c4 as i32, c3 as i32 - 2 * c6 as i32)
                                            ..min(16 * c4 as i32 + 15, c3 as i32 - c6 as i32 - 2)
                                        {
                                            m.write().unwrap()[(
                                                c3 - 2 * c6 as usize,
                                                (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                            )] = minsq(
                                                &s,
                                                m.read().unwrap()
                                                    [(c3 - 2 * c6 as usize, c10 as usize)]
                                                    + m.read().unwrap()[(
                                                        c10 as usize + 1,
                                                        (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                                    )]
                                                    + p[((c3 - 2 * c6 as usize) - 1)]
                                                        * p[c10 as usize]
                                                        * p[(c3 - 2 * c6 as usize) + c6 as usize
                                                            - 1],
                                                m.read().unwrap()[(
                                                    c3 - 2 * c6 as usize,
                                                    (c3 - 2 * c6 as usize) + c6 as usize - 1,
                                                )],
                                                c3 - 2 * c6 as usize,
                                                (c3 - 2 * c6 as usize) + c6 as usize,
                                                c10,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            }
        }
    }
    let stop = start.elapsed().as_secs_f64();
    println!("It took {stop} seconds");
    return m.read().unwrap()[(1, n - 1)];
}
fn main() {
    let mut n = 1500;
    let mut dim = 1502;
    let mut num_proc = 4;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(value) => num_proc = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    let mut kind = 1;
    if args.len() > 2 {
        match args[2].parse::<usize>() {
            Ok(value) => n = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    dim = n + 10;
    if args.len() > 3 {
        match args[3].parse::<i32>() {
            Ok(value) => kind = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    let thread_build: Result<ThreadPool, ThreadPoolBuildError> =
        ThreadPoolBuilder::new().num_threads(num_proc).build();
    if let Err(e) = &thread_build {
        eprintln!("{:?}", e);
    }
    let mut p: Vec<i32> = vec![];
    for i in 0..n {
        p.push((i as i32 % 20) + 1);
    }
    println!(
        "Minimum number of multiplications is {}",
        matrix_chain_order(p, n, kind, dim)
    );
}
