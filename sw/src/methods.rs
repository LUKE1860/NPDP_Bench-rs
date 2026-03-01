use crate::mem::TSMatrix;
use std::cmp::{max, min};
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn ceild(f_num: f64, s_num: f64) -> usize {
    return f64::ceil(f_num / s_num) as usize;
}
pub fn s(x: char, z: char) -> i32 {
    if x == z {
        return 1;
    }
    return -1;
}
pub fn sw_seq(
    m1: &mut TSMatrix<i32>,
    m2: &mut TSMatrix<i32>,
    h: &mut TSMatrix<i32>,
    w: &mut Vec<i32>,
    a: &mut Vec<char>,
    b: &mut Vec<char>,
    n: usize,
) {
    rayon::scope(|x| {
        x.spawn(|_| {
            for i in 1..=n {
                for j in 1..=n {
                    m1.write().unwrap()[(i, j)] = i32::MIN;
                    for k in 1..=i {
                        m1.write().unwrap()[(i, j)] = max(
                            m1.read().unwrap()[(i, j)],
                            h.read().unwrap()[(i - k, j)] + w[k],
                        );
                    }
                    m2.write().unwrap()[(i, j)] = i32::MIN;
                    for k in 1..=j {
                        m2.write().unwrap()[(i, j)] = max(
                            m2.read().unwrap()[(i, j)],
                            h.read().unwrap()[(i - k, j)] + w[k],
                        );
                    }
                    h.write().unwrap()[(i, j)] = max(
                        0,
                        max(
                            h.read().unwrap()[(i - 1, j - 1)] + s(a[i], b[i]),
                            max(m1.read().unwrap()[(i, j)], m2.read().unwrap()[(i, j)]),
                        ),
                    );
                }
            }
        });
    });
}
pub fn sw_pluto(
    m1: &mut TSMatrix<i32>,
    m2: &mut TSMatrix<i32>,
    h: &mut TSMatrix<i32>,
    w: &mut Vec<i32>,
    a: &mut Vec<char>,
    b: &mut Vec<char>,
    n: usize,
) {
    if n >= 1 {
        let lbp = 0;
        let ubp = floord(n as f64, 16f64);
        rayon::broadcast(|_| {
            for t2 in lbp..ubp {
                for t3 in 0..floord(n as f64, 16f64) {
                    for t4 in max(1, 16 * t2)..=min(n, 16 * t2 + 15) {
                        let lbv = max(1, 16 * t3);
                        let ubv = min(n, 16 * t3 + 15);
                        for t5 in lbv..=ubv {
                            m2.write().unwrap()[(t4, t5)] = i32::MIN;
                            m1.write().unwrap()[(t4, t5)] = i32::MIN;
                        }
                    }
                }
            }
            for t2 in 0..=floord(n as f64, 8f64) {
                let lbp = max(0, ceild((16 * t2 - n) as f64, 16 as f64));
                let ubp = min(floord(n as f64, 16f64), t2);
                rayon::broadcast(|_| {
                    for t3 in lbp..=ubp {
                        if t2 >= 2 * t3 + 1 {
                            for t4 in 16 * t2 - 16 * t3..=min(n, 16 * t2 - 16 * t3 + 15) {
                                for t5 in max(1, 16 * t3)..=16 * t3 + 15 {
                                    for t6 in t5 + 1..=t4 {
                                        m1.write().unwrap()[(t4, t5)] = max(
                                            m1.read().unwrap()[(t4, t5)],
                                            h.read().unwrap()[(
                                                t4 - (h.read().unwrap().row_len() - t5 + t6),
                                                t5,
                                            )] + w[h.read().unwrap().row_len() - t5 + t6],
                                        );
                                    }
                                    for t6 in t4 + 1..=t4 + t5 {
                                        m2.write().unwrap()[(t4, t5)] = max(
                                            m1.read().unwrap()[(t4, t5)],
                                            h.read().unwrap()[(
                                                t4,
                                                t5 - (h.read().unwrap().row_len() - t4 + t6),
                                            )] + w[h.read().unwrap().row_len() - t5 + t6],
                                        );
                                        m1.write().unwrap()[(t4, t5)] = max(
                                            m1.read().unwrap()[(t4, t5)],
                                            h.read().unwrap()[(
                                                t4 - (-(h.read().unwrap().row_len() as i32
                                                    - t5 as i32)
                                                    + t6 as i32)
                                                    as usize,
                                                t5,
                                            )] + w[h.read().unwrap().row_len() - t5 + t6],
                                        );
                                    }
                                    h.write().unwrap()[(t4, t5)] = max(
                                        0,
                                        max(
                                            h.read().unwrap()[(t4 - 1, t5 - 1)] + s(a[t4], b[t4]),
                                            max(
                                                m1.read().unwrap()[(t4, t5)],
                                                m2.read().unwrap()[(t4, t5)],
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                        if n >= 2 && t2 == 0 && t3 == 0 {
                            m2.write().unwrap()[(1, 1)] =
                                max(m2.read().unwrap()[(1, 1)], h.read().unwrap()[(1, 0)] + w[1]);
                            m1.write().unwrap()[(1, 1)] =
                                max(m1.read().unwrap()[(1, 1)], h.read().unwrap()[(0, 1)] + w[1]);
                            h.write().unwrap()[(1, 1)] = max(
                                0,
                                max(
                                    h.read().unwrap()[(0, 0)] + s(a[1], b[1]),
                                    max(m1.read().unwrap()[(1, 1)], m2.read().unwrap()[(1, 1)]),
                                ),
                            );
                            for t5 in 2..=min(15, n) {
                                for t6 in 2..=t5 {
                                    m2.write().unwrap()[(1, t5)] = max(
                                        m2.read().unwrap()[(1, t5)],
                                        h.read().unwrap()[(1, t5 - (t6 - 1))] + w[t6 - 1],
                                    );
                                }
                                m2.write().unwrap()[(1, t5)] = max(
                                    m2.read().unwrap()[(1, t5)],
                                    h.read().unwrap()[(1, 0)] + w[t5],
                                );
                                m1.write().unwrap()[(1, t5)] = max(
                                    m1.read().unwrap()[(1, t5)],
                                    h.read().unwrap()[(0, t5)] + w[1],
                                );
                                h.write().unwrap()[(1, 1)] = max(
                                    0,
                                    max(
                                        h.read().unwrap()[(0, t5 - 1)] + s(a[1], b[1]),
                                        max(
                                            m1.read().unwrap()[(1, t5)],
                                            m2.read().unwrap()[(1, t5)],
                                        ),
                                    ),
                                );
                            }
                        }
                        if n == 1 && t2 == 0 && t3 == 0 {
                            m2.write().unwrap()[(1, 1)] =
                                max(m2.read().unwrap()[(1, 1)], h.read().unwrap()[(1, 0)] + w[1]);
                            m1.write().unwrap()[(1, 1)] =
                                max(m1.read().unwrap()[(1, 1)], h.read().unwrap()[(0, 1)] + w[1]);
                            h.write().unwrap()[(1, 1)] = max(
                                0,
                                max(
                                    h.read().unwrap()[(0, 0)] + s(a[1], b[1]),
                                    max(m1.read().unwrap()[(1, 1)], m2.read().unwrap()[(1, 1)]),
                                ),
                            );
                        }
                        if t2 >= 2 && t2 == 2 * t3 && t2 <= floord((n - 1) as f64, 8f64) {
                            for t6 in 8 * t2 + 1..=16 * t2 {
                                if t2 % 2 == 0 {
                                    m2.write().unwrap()[(8 * t2, 8 * t2)] = max(
                                        m2.read().unwrap()[(8 * t2, 8 * t2)],
                                        h.read().unwrap()[(
                                            8 * t2,
                                            8 * t2 - (-8 * t2 as i32 + t6 as i32) as usize,
                                        )] + w[(-8 * t2 as i32 + t6 as i32) as usize],
                                    );
                                    m1.write().unwrap()[(8 * t2, 8 * t2)] = max(
                                        m1.read().unwrap()[(8 * t2, 8 * t2)],
                                        h.read().unwrap()[(
                                            8 * t2 - (-8 * t2 as i32 + t6 as i32) as usize,
                                            8 * t2,
                                        )] + w[(-8 * t2 as i32 + t6 as i32) as usize],
                                    );
                                }
                            }
                            //74 line
                            if t2 % 2 == 0 {
                                h.write().unwrap()[(8 * t2, 8 * t2)] = max(
                                    0,
                                    max(
                                        h.read().unwrap()[(8 * t2 - 1, 8 * t2 - 1)]
                                            + s(a[8 * t2], b[8 * t2]),
                                        max(
                                            m1.read().unwrap()[(8 * t2, 8 * t2)],
                                            m2.read().unwrap()[(1, 1)],
                                        ),
                                    ),
                                );
                            }
                            for t5 in 8 * t2 + 1..=min(n, 8 * t2 + 15) {
                                for t6 in 8 * t2 + 1..=t5 {
                                    if t2 % 2 == 0 {
                                        m2.write().unwrap()[(8 * t2, t5)] = max(
                                            m2.read().unwrap()[(8 * t2, t5)],
                                            h.read().unwrap()[(
                                                8 * t2,
                                                t5 - (-8 * t2 as i32 + t6 as i32) as usize,
                                            )] + w[(-8 * t2 as i32 + t6 as i32) as usize],
                                        );
                                    }
                                }
                                for t6 in t5 + 1..=8 * t2 + t5 {
                                    if t2 % 2 == 0 {
                                        m2.write().unwrap()[(8 * t2, t5)] = max(
                                            m2.read().unwrap()[(8 * t2, t5)],
                                            h.read().unwrap()[(
                                                8 * t2,
                                                t5 - (-8 * t2 as i32 + t6 as i32) as usize,
                                            )] + w[(-8 * t2 as i32 + t6 as i32) as usize],
                                        );
                                        m1.write().unwrap()[(8 * t2, t5)] = max(
                                            m2.read().unwrap()[(8 * t2, t5)],
                                            h.read().unwrap()[(
                                                8 * t2 - (-(t5 as i32) + t6 as i32) as usize,
                                                t5,
                                            )] + w[(-(t5 as i32) + t6 as i32) as usize],
                                        );
                                        h.write().unwrap()[(8 * t2, t5)] = max(
                                            0,
                                            max(
                                                h.read().unwrap()[(8 * t2 - 1, t5 - 1)]
                                                    + s(a[8 * t2], b[8 * t2]),
                                                max(
                                                    m1.read().unwrap()[(8 * t2, t5)],
                                                    m2.read().unwrap()[(8 * t2, t5)],
                                                ),
                                            ),
                                        );
                                    }
                                }
                            }
                            if 8 * t2 == n && 16 * t3 == n {
                                for t6 in n + 1..=2 * n {
                                    if n % 16 == 0 {
                                        m2.write().unwrap()[(n, n)] = max(
                                            m2.read().unwrap()[(n, n)],
                                            h.read().unwrap()[(n, n - (t6 - n))] + w[t6 - n],
                                        );
                                        m1.write().unwrap()[(n, n)] = max(
                                            m1.read().unwrap()[(n, n)],
                                            h.read().unwrap()[(n - (t6 - n), n)] + w[t6 - n],
                                        );
                                        h.write().unwrap()[(n, n)] = max(
                                            0,
                                            max(
                                                h.read().unwrap()[(n - 1, n - 1)] + s(a[n], b[n]),
                                                max(
                                                    m1.read().unwrap()[(n, n)],
                                                    m2.read().unwrap()[(n, n)],
                                                ),
                                            ),
                                        );
                                    }
                                    if t2 <= 2 * t3 - 1 {
                                        for t4 in max(1, 16 * t2 - 16 * t3)..=16 * t2 - 16 * t3 + 15
                                        {
                                            for t5 in 16 * t3..min(n, 16 * t3 + 15) {
                                                for t6 in t4 + 1..=t5 {
                                                    m2.write().unwrap()[(t4, t5)] = max(
                                                        m2.read().unwrap()[(t4, t5)],
                                                        h.read().unwrap()[(
                                                            t4,
                                                            t5 - (-(t4 as i32) + t6 as i32)
                                                                as usize,
                                                        )] + w[(-(t4 as i32) + t6 as i32) as usize],
                                                    );
                                                }
                                                for t6 in t5 + 1..=t4 + t5 {
                                                    m2.write().unwrap()[(t4, t5)] = max(
                                                        m2.read().unwrap()[(t4, t5)],
                                                        h.read().unwrap()[(
                                                            t4,
                                                            t5 - (-(t4 as i32) + t6 as i32)
                                                                as usize,
                                                        )] + w[(-(t4 as i32) + t6 as i32) as usize],
                                                    );
                                                    m1.write().unwrap()[(t4, t5)] = max(
                                                        m2.read().unwrap()[(t4, t5)],
                                                        h.read().unwrap()[(
                                                            t4 - (-(t5 as i32) + t6 as i32)
                                                                as usize,
                                                            t5,
                                                        )] + w[(-(t5 as i32) + t6 as i32) as usize],
                                                    );
                                                }
                                                h.write().unwrap()[(t4, t5)] = max(
                                                    0,
                                                    max(
                                                        h.read().unwrap()[(t4 - 1, t5 - 1)]
                                                            + s(a[t4], b[t4]),
                                                        max(
                                                            m1.read().unwrap()[(t4, t5)],
                                                            m2.read().unwrap()[(t4, t5)],
                                                        ),
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    if t2 == 2 * t3 {
                                        for t4 in max(2, 8 * t2 + 1)..=min(n - 1, 8 * t2 + 14) {
                                            for t5 in max(1, 8 * t2)..=t4 - 1 {
                                                for t6 in t5 + 1..=t4 {
                                                    if t2 % 2 == 0 {
                                                        m1.write().unwrap()[(t4, t5)] = max(
                                                            m1.read().unwrap()[(t4, t5)],
                                                            h.read().unwrap()[(
                                                                (t4 - (-(t5 as i32) + t6 as i32)
                                                                    as usize),
                                                                t5,
                                                            )] + w[(-(t5 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                    }
                                                }
                                                for t6 in t4 + 1..=t4 + t5 {
                                                    if t2 % 2 == 0 {
                                                        m2.write().unwrap()[(t4, t5)] = max(
                                                            m2.read().unwrap()[(t4, t5)],
                                                            h.read().unwrap()[(
                                                                t4,
                                                                t5 - (-(t4 as i32) + t6 as i32)
                                                                    as usize,
                                                            )] + w[(-(t4 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                        m1.write().unwrap()[(t4, t5)] = max(
                                                            m1.read().unwrap()[(t4, t5)],
                                                            h.read().unwrap()[(
                                                                (t4 - (-(t5 as i32) + t6 as i32)
                                                                    as usize),
                                                                t5,
                                                            )] + w[(-(t5 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                        h.write().unwrap()[(t4, t5)] = max(
                                                            0,
                                                            max(
                                                                h.read().unwrap()[(t4 - 1, t5 - 1)]
                                                                    + s(a[t4], b[t4]),
                                                                max(
                                                                    m1.read().unwrap()[(t4, t5)],
                                                                    m2.read().unwrap()[(t4, t5)],
                                                                ),
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                            for t6 in t4 + 1..=2 * t4 {
                                                if t2 % 2 == 0 {
                                                    m2.write().unwrap()[(t4, t4)] = max(
                                                        m2.read().unwrap()[(t4, t4)],
                                                        h.read().unwrap()[(
                                                            t4,
                                                            t4 - (-(t4 as i32) + t6 as i32)
                                                                as usize,
                                                        )] + w[(-(t4 as i32) + t6 as i32) as usize],
                                                    );
                                                    m1.write().unwrap()[(t4, t4)] = max(
                                                        m1.read().unwrap()[(t4, t4)],
                                                        h.read().unwrap()[(
                                                            (t4 - (-(t4 as i32) + t6 as i32)
                                                                as usize),
                                                            t4,
                                                        )] + w[(-(t4 as i32) + t6 as i32) as usize],
                                                    );
                                                }
                                            }
                                            if t2 % 2 == 0 {
                                                h.write().unwrap()[(t4, t4)] = max(
                                                    0,
                                                    max(
                                                        h.read().unwrap()[(t4 - 1, t4 - 1)]
                                                            + s(a[t4], b[t4]),
                                                        max(
                                                            m1.read().unwrap()[(t4, t4)],
                                                            m2.read().unwrap()[(t4, t4)],
                                                        ),
                                                    ),
                                                );
                                            }
                                            for t5 in t4 + 1..=min(n, 8 * t2 + 15) {
                                                for t6 in t4 + 1..=t5 {
                                                    if t2 % 2 == 0 {
                                                        m2.write().unwrap()[(t4, t5)] = max(
                                                            m2.read().unwrap()[(t4, t5)],
                                                            h.read().unwrap()[(
                                                                t4,
                                                                t5 - (-(t4 as i32) + t6 as i32)
                                                                    as usize,
                                                            )] + w[(-(t4 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                    }
                                                }
                                                //160
                                                for t6 in t5 + 1..=t4 + t5 {
                                                    if t2 % 2 == 0 {
                                                        m2.write().unwrap()[(t4, t5)] = max(
                                                            m2.read().unwrap()[(t4, t5)],
                                                            h.read().unwrap()[(
                                                                t4,
                                                                t5 - (-(t4 as i32) + t6 as i32)
                                                                    as usize,
                                                            )] + w[(-(t4 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                        m1.write().unwrap()[(t4, t5)] = max(
                                                            m1.read().unwrap()[(t4, t5)],
                                                            h.read().unwrap()[(
                                                                (t4 - (-(t5 as i32) + t6 as i32)
                                                                    as usize),
                                                                t5,
                                                            )] + w[(-(t5 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                    }
                                                }
                                                if t2 % 2 == 0 {
                                                    h.write().unwrap()[(t4, t5)] = max(
                                                        0,
                                                        max(
                                                            h.read().unwrap()[(t4 - 1, t5 - 1)]
                                                                + s(a[t4], b[t4]),
                                                            max(
                                                                m1.read().unwrap()[(t4, t5)],
                                                                m2.read().unwrap()[(t4, t5)],
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        if n >= 2
                                            && t2 == 2 * t3
                                            && t2 <= floord((n - 1) as f64, 8f64)
                                            && t2 >= ceild((n - 14) as f64, 8f64)
                                        {
                                            for t5 in max(1, 8 * t2)..=n - 1 {
                                                for t6 in t5 + 1..=n {
                                                    if t2 % 2 == 0 {
                                                        m1.write().unwrap()[(n, t5)] = max(
                                                            m1.read().unwrap()[(n, t5)],
                                                            h.read().unwrap()[(
                                                                (n - (-(t5 as i32) + t6 as i32)
                                                                    as usize),
                                                                t5,
                                                            )] + w[(-(t5 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                    }
                                                }
                                                for t6 in n + 1..=t5 + n {
                                                    if t2 % 2 == 0 {
                                                        m2.write().unwrap()[(n, t5)] = max(
                                                            m2.read().unwrap()[(n, t5)],
                                                            h.read().unwrap()[(n, t5 - (t6 - n))]
                                                                + w[t6 - n],
                                                        );
                                                        m1.write().unwrap()[(n, t5)] = max(
                                                            m1.read().unwrap()[(n, t5)],
                                                            h.read().unwrap()[(
                                                                n - (-(t5 as i32) + t6 as i32)
                                                                    as usize,
                                                                t5,
                                                            )] + w[(-(t5 as i32) + t6 as i32)
                                                                as usize],
                                                        );
                                                    }
                                                }
                                                if t2 % 2 == 0 {
                                                    h.write().unwrap()[(n, t5)] = max(
                                                        0,
                                                        max(
                                                            h.read().unwrap()[(n - 1, n - 1)]
                                                                + s(a[n], b[n]),
                                                            max(
                                                                m1.read().unwrap()[(n, t5)],
                                                                m2.read().unwrap()[(n, t5)],
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }
                                            for t6 in n + 1..=2 * n {
                                                if t2 % 2 == 0 {
                                                    m2.write().unwrap()[(n, n)] = max(
                                                        m2.read().unwrap()[(n, n)],
                                                        h.read().unwrap()[(n, n - (t6 - n))]
                                                            + w[t6 - n],
                                                    );
                                                    m1.write().unwrap()[(n, n)] = max(
                                                        m1.read().unwrap()[(n, n)],
                                                        h.read().unwrap()[(n - (t6 - n), n)]
                                                            + w[t6 - n],
                                                    );
                                                }
                                            }
                                            if t2 % 2 == 0 {
                                                h.write().unwrap()[(n, n)] = max(
                                                    0,
                                                    max(
                                                        h.read().unwrap()[(n - 1, n - 1)]
                                                            + s(a[n], b[n]),
                                                        max(
                                                            m1.read().unwrap()[(n, n)],
                                                            m2.read().unwrap()[(n, n)],
                                                        ),
                                                    ),
                                                );
                                            }
                                        }
                                        if t2 == 2 * t3 && t2 <= floord((n - 15) as f64, 8f64) {
                                            for t5 in max(1, 8 * t2)..=8 * t2 + 14 {
                                                for t6 in t5 + 1..=8 * t2 + 15 {
                                                    if t2 % 2 == 0 {
                                                        m1.write().unwrap()[(8 * t2 + 15, t5)] =
                                                            max(
                                                                m1.read().unwrap()
                                                                    [(8 * t2 + 15, t5)],
                                                                h.read().unwrap()[(
                                                                    8 * t2 + 15
                                                                        - (-(t5 as i32)
                                                                            + (t6 as i32))
                                                                            as usize,
                                                                    t5,
                                                                )] + w[(-(t5 as i32) + t6 as i32)
                                                                    as usize],
                                                            );
                                                    }
                                                }
                                                for t6 in 8 * t2 + 16..=8 * t2 + t5 + 15 {
                                                    if t2 % 2 == 0 {
                                                        m2.write().unwrap()[(8 * t2 + 15, t5)] =
                                                            max(
                                                                m2.read().unwrap()
                                                                    [(8 * t2 + 15, t5)],
                                                                h.read().unwrap()[(
                                                                    8 * t2 + 15,
                                                                    t5 - (-8 * t2 as i32
                                                                        + t6 as i32
                                                                        - 15)
                                                                        as usize,
                                                                )] + w[(-8 * t2 as i32 + t6 as i32
                                                                    - 15)
                                                                    as usize],
                                                            );
                                                        m1.write().unwrap()[(8 * t2 + 15, t5)] =
                                                            max(
                                                                m2.read().unwrap()
                                                                    [(8 * t2 + 15, t5)],
                                                                h.read().unwrap()[(
                                                                    8 * t2 + 15
                                                                        - (-(t5 as i32) + t6 as i32)
                                                                            as usize,
                                                                    t5,
                                                                )] + w[(-(t5 as i32) + t6 as i32)
                                                                    as usize],
                                                            );
                                                    }
                                                }
                                                if t2 % 2 == 0 {
                                                    h.write().unwrap()[(8 * t2 + 15, t5)] = max(
                                                        0,
                                                        max(
                                                            h.read().unwrap()
                                                                [((8 * t2 + 15) - 1, t5 - 1)]
                                                                + s(a[8 * t2 + 15], b[8 * t2 + 15]),
                                                            max(
                                                                m1.read().unwrap()
                                                                    [(8 * t2 + 15, t5)],
                                                                m2.read().unwrap()
                                                                    [(8 * t2 + 15, t5)],
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }
                                            for t6 in 8 * t2 + 16..=16 * t2 + 30 {
                                                if t2 % 2 == 0 {
                                                    m2.write().unwrap()
                                                        [(8 * t2 + 15, 8 * t2 + 15)] = max(
                                                        m2.read().unwrap()
                                                            [(8 * t2 + 15, 8 * t2 + 15)],
                                                        h.read().unwrap()[(
                                                            8 * t2 + 15,
                                                            (8 * t2 + 15)
                                                                - (-8 * t2 as i32 + t6 as i32 - 15)
                                                                    as usize,
                                                        )] + w[(-8 * t2 as i32 + t6 as i32 - 15)
                                                            as usize],
                                                    );
                                                    m1.write().unwrap()
                                                        [(8 * t2 + 15, 8 * t2 + 15)] = max(
                                                        m1.read().unwrap()
                                                            [(8 * t2 + 15, 8 * t2 + 15)],
                                                        h.read().unwrap()[(
                                                            (8 * t2 + 15)
                                                                - (-8 * t2 as i32 + t6 as i32 - 15)
                                                                    as usize,
                                                            8 * t2 + 15,
                                                        )] + w[(-8 * t2 as i32 + t6 as i32 - 15)
                                                            as usize],
                                                    );
                                                }
                                            }
                                            if t2 % 2 == 0 {
                                                h.write().unwrap()[(8 * t2 + 15, 8 * t2 + 15)] =
                                                    max(
                                                        0,
                                                        max(
                                                            h.read().unwrap()[(
                                                                (8 * t2 + 15) - 1,
                                                                (8 * t2 + 15) - 1,
                                                            )] + s(a[8 * t2 + 15], b[8 * t2 + 15]),
                                                            max(
                                                                m1.read().unwrap()
                                                                    [(8 * t2 + 15, 8 * t2 + 15)],
                                                                m2.read().unwrap()
                                                                    [(8 * t2 + 15, 8 * t2 + 15)],
                                                            ),
                                                        ),
                                                    );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            }
        });
    }
}
pub fn sw_traco(
    m1: &mut TSMatrix<i32>,
    m2: &mut TSMatrix<i32>,
    h: &mut TSMatrix<i32>,
    w: &mut Vec<i32>,
    a: &mut Vec<char>,
    b: &mut Vec<char>,
    n: usize,
) {
    rayon::broadcast(|_| {
        for c1 in 0..=(n - 1) / 16 {
            for c3 in 0..=(n - 1) / 16 {
                for c5 in 16 * c1 + 1..=min(n, 16 * c1 + 16) {
                    for c7 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                        m1.write().unwrap()[(c5, c7)] = i32::MIN;
                        m2.write().unwrap()[(c5, c7)] = i32::MIN;
                    }
                }
            }
        }
    });
    for c1 in 0..=floord((n - 1) as f64, 8f64) {
        rayon::broadcast(|_| {
            for c3 in max(0, c1 - (n + 15) / 16 + 1)..=min(c1, (n - 1) / 16) {
                for c4 in 0..=2 {
                    if c4 == 2 {
                        for c7 in 16 * c1 - 16 * c3 + 1..=min(n, 16 * c1 - 16 * c3 + 16) {
                            for c9 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                                for c10 in max(0, 16 * c1 - 16 * c3 - c7 + 2)
                                    ..=min(1, (-16 * c3 as i32 + c9 as i32 - 1) as usize)
                                {
                                    if c10 == 1 {
                                        for c11 in 1..=c9 {
                                            m2.write().unwrap()[(c7, c9)] = max(
                                                m2.read().unwrap()[(c7, c9)]
                                                    + h.read().unwrap()[(c7, c9 - c11)],
                                                w[c11],
                                            );
                                        }
                                    } else {
                                        for c11 in 1..=c7 {
                                            m1.write().unwrap()[(c7, c9)] = max(
                                                m1.read().unwrap()[(c7, c9)]
                                                    + h.read().unwrap()[(c7 - c11, c9)],
                                                w[c11],
                                            );
                                        }
                                    }
                                    h.write().unwrap()[(c7, c9)] = max(
                                        0,
                                        max(
                                            h.read().unwrap()[(c7 - 1, c9 - 1)] + s(a[c7], b[c9]),
                                            max(
                                                m1.read().unwrap()[(c7, c9)],
                                                m2.read().unwrap()[(c7, c9)],
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                    } else if c4 == 1 {
                        for c5 in 0..=c3 {
                            for c7 in 16 * c1 - 16 * c3 + 1..min(n, 16 * c1 - 16 * c3 + 16) {
                                for c11 in 16 * c5 + 1..=min(16 * c3 + 1, 16 * c5 + 16) {
                                    m2.write().unwrap()[(c7, 16 * c3 + 1)] = max(
                                        m2.read().unwrap()[(c7, 16 * c3 + 1)]
                                            + h.read().unwrap()[(c7, (16 * c3 + 1) - c11)],
                                        w[c11],
                                    );
                                }
                            }
                        }
                    } else {
                        for c5 in 0..=c1 - c3 {
                            for c9 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                                for c11 in 16 * c5 + 1..min(16 * c1 - 16 * c3 + 1, 16 * c5 + 16) {
                                    m1.write().unwrap()[(16 * c1 - 16 * c3 + 1, c9)] = max(
                                        m1.read().unwrap()[(16 * c1 - 16 * c3 + 1, c9)]
                                            + h.read().unwrap()
                                                [((16 * c1 - 16 * c3 + 1) - c11, c9)],
                                        w[c11],
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
pub fn sw_tstile(
    m1: &mut TSMatrix<i32>,
    m2: &mut TSMatrix<i32>,
    h: &mut TSMatrix<i32>,
    w: &mut Vec<i32>,
    a: &mut Vec<char>,
    b: &mut Vec<char>,
    n: usize,
) {
    rayon::broadcast(|_| {
        for c1 in 0..=(n - 1) / 16 {
            for c3 in 0..=(n - 1) / 16 {
                for c5 in 16 * c1 + 1..=min(n, 16 * c1 + 16) {
                    for c7 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                        m1.write().unwrap()[(c5, c7)] = i32::MIN;
                        m2.write().unwrap()[(c5, c7)] = i32::MIN;
                    }
                }
            }
        }
    });
    for c0 in 0..floord((n - 1) as f64, 8f64) {
        rayon::broadcast(|_| {
            for c1 in max(0, c0 - (n + 15) / 16 + 1)..=min(c0, (n - 1) / 16) {
                for c3 in 16 * c0 + 2
                    ..=min(
                        min(min(2 * n, 16 * c0 + 32), n + 16 * c1 + 16),
                        n + 16 * c0 - 16 * c1 + 16,
                    )
                {
                    for c4 in max(
                        max(
                            (-(c0 as i32) + c1 as i32 - 1) as usize,
                            (-(n as i32 + 14)) as usize / 16,
                        ),
                        c1 - (c3 + 13) / 16,
                    )..c0 - c1 - (c3 + 13) / 16
                    {
                        for c6 in max(
                            max(
                                16 * c1 + 1,
                                (-16 * c0 as i32 + 16 * c1 as i32 + c3 as i32 - 16) as usize,
                            ),
                            -(n as i32) as usize + c3,
                        )
                            ..=min(
                                min(
                                    16 * c1 + 16,
                                    (-16 * c0 as i32 + 16 * c1 as i32 + c3 as i32 - 1) as usize,
                                ),
                                c3 + 16 * c4 + 14,
                            )
                        {
                            for c10 in max(1, c3 + 16 * c4 - c6)..=c3 + 16 * c4 - c6 + 15 {
                                m2.write().unwrap()[(c6, c3 - c6)] = max(
                                    m2.read().unwrap()[(c6, c3 - c6)],
                                    h.read().unwrap()[(c6, (c3 - c6) - c10)] + w[c10],
                                );
                            }
                        }
                    }
                    if c0 >= 2 * c1 + 1 && c3 >= 16 * c0 + 19 {
                        for c6 in max(
                            (-16 * c0 as i32 + 16 * c1 as i32) as usize + c3 - 16,
                            (-(n as i32) + c3 as i32) as usize,
                        )..16 * c1 + 16
                        {
                            for c10 in
                                max(1, (-16 * c1 as i32 + c3 as i32 - c6 as i32 - 32) as usize)
                                    ..(-16 * c1 as i32 + c3 as i32 - c6 as i32 - 16) as usize
                            {
                                m2.write().unwrap()[(c6, c3 - c6)] = max(
                                    m2.read().unwrap()[(c6, c3 - c6)],
                                    h.read().unwrap()[(c6, (c3 - c6) - c10)] + w[c10],
                                );
                            }
                        }
                    }
                    for c4 in max(
                        max(
                            (-(c1 as i32) - 1) as usize,
                            (-(n as i32 + 14) as usize / 16),
                        ),
                        c0 - c1 - (c3 + 13) / 16,
                    )..=0
                    {
                        if n + 16 * c1 + 1 >= c3 && 16 * c0 + 17 >= c3 && (c1 + c4) as i32 == -1 {
                            for c10 in max(1, (-32 * c1 as i32 + c3 as i32 - 17) as usize)
                                ..((-32 * c1 as i32 + c3 as i32) as usize - 1)
                            {
                                m2.write().unwrap()
                                    [(16 * c1 + 1, (-16 * c1 as i32 + c3 as i32 - 1) as usize)] =
                                    max(
                                        m2.read().unwrap()[(
                                            16 * c1 + 1,
                                            (-16 * c1 as i32 + c3 as i32 - 1) as usize,
                                        )],
                                        h.read().unwrap()[(
                                            16 * c1 + 1,
                                            (-16 * c1 as i32 + c3 as i32 - 1) as usize - c10,
                                        )] + w[c10],
                                    );
                            }
                        }
                        for c6 in max(
                            max(
                                max(16 * c1 + 1, (-16 * c0 as i32) as usize + 16 * c1 + c3 - 16),
                                -(n as i32) as usize + c3,
                            ),
                            (-16 * c4 as i32) as usize - 14,
                        )
                            ..=min(
                                min(n, 16 * c1 + 16),
                                (-16 * c0 as i32 + 16) as usize * c1 + c3 - 1,
                            )
                        {
                            for c10 in max(1, 16 * c4 + c6)..=min(c6, 16 * c4 + c6 + 15) {
                                m1.write().unwrap()[(c6, c3 - c6)] = max(
                                    m1.read().unwrap()[(c6, c3 - c6)],
                                    h.read().unwrap()[(c6 - c10, c3 - c6)] + w[c10],
                                );
                            }
                            for c10 in
                                max(1, c3 + 16 * c4 - c6)..=min(c3 - c6, c3 + 16 * c4 - c6 + 15)
                            {
                                m2.write().unwrap()[(c6, c3 - c6)] = max(
                                    m2.read().unwrap()[(c6, c3 - c6)],
                                    h.read().unwrap()[(c6, (c3 - c6) - c10)] + w[c10],
                                );
                            }
                            if c0 == 0 && c1 == 0 && c3 <= 15 && c4 == 0 {
                                h.write().unwrap()[(c6, c3 - c6)] = max(
                                    0,
                                    max(
                                        h.read().unwrap()[(c6 - 1, (c3 - c6) - 1)]
                                            + s(a[c6], b[c6]),
                                        max(
                                            m1.read().unwrap()[(c6, c3 - c6)],
                                            m2.read().unwrap()[(c6, c3 - c6)],
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                    if c3 >= 16 {
                        for c6 in max(
                            max(16 * c1 + 1, (-16 * c0 as i32 + 16) as usize * c1 + c3 - 16),
                            (-(n as i32) + c3 as i32) as usize,
                        )
                            ..=min(
                                min(n, 16 * c1 + 16),
                                (-16 * c0 as i32 + 16 * c1 as i32 + c3 as i32 - 1) as usize,
                            )
                        {
                            h.write().unwrap()[(c6, c3 - c6)] = max(
                                0,
                                max(
                                    h.read().unwrap()[(c6 - 1, (c3 - c6) - 1)] + s(a[c6], b[c6]),
                                    max(
                                        m1.read().unwrap()[(c6, c3 - c6)],
                                        m2.read().unwrap()[(c6, c3 - c6)],
                                    ),
                                ),
                            );
                        }
                    }
                }
            }
        });
    }
}
