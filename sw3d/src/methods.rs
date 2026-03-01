use crate::mem3d::TSMatrix3d;
use std::{
    cmp::{max, min},
    i32,
};
pub fn s(x: char, z: char) -> i32 {
    if x == z {
        return 1;
    }
    return -1;
}
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn ceild(f_num: f64, s_num: f64) -> usize {
    return f64::ceil(f_num / s_num) as usize;
}
pub fn sw_seq(
    m1: &mut TSMatrix3d<i32>,
    m2: &mut TSMatrix3d<i32>,
    m3: &mut TSMatrix3d<i32>,
    m4: &mut TSMatrix3d<i32>,
    m5: &mut TSMatrix3d<i32>,
    m6: &mut TSMatrix3d<i32>,
    h: &mut TSMatrix3d<i32>,
    w: &Vec<i32>,
    a: &Vec<char>,
    b: &Vec<char>,
    c: &Vec<char>,
    n: usize,
) {
    rayon::scope(|x| {
        x.spawn(|_| {
            for i in 1..=n {
                for j in 1..=n {
                    for l in 1..=n {
                        m1.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=i {
                            m1.write().unwrap()[[i, j, l]] = max(
                                m1.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i - k, j, l]] - 2 * w[k],
                            );
                        }
                        m2.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=j {
                            m2.write().unwrap()[[i, j, l]] = max(
                                m2.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i, j - k, l]] - 2 * w[k],
                            );
                        }
                        m3.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=l {
                            m3.write().unwrap()[[i, j, l]] = max(
                                m3.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i, j, l - k]] - 2 * w[k],
                            );
                        }
                        m4.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=min(i, j) {
                            m4.write().unwrap()[[i, j, l]] = max(
                                m4.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i - k, j - k, l]] - w[k] + s(a[i], b[j]),
                            );
                        }
                        m5.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=min(j, l) {
                            m5.write().unwrap()[[i, j, l]] = max(
                                m5.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i, j - k, l - k]] - w[k] + s(b[j], c[l]),
                            );
                        }
                        m6.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=min(i, l) {
                            m6.write().unwrap()[[i, j, l]] = max(
                                m6.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i - k, j, l - k]] - w[k] + s(a[i], c[l]),
                            );
                        }
                        h.write().unwrap()[[i, j, l]] = max(
                            0,
                            max(
                                h.read().unwrap()[[i - 1, j - 1, l - 1]]
                                    + s(a[i], b[j])
                                    + s(a[i], c[l])
                                    + s(b[j], c[l]),
                                max(
                                    m1.read().unwrap()[[i, j, l]],
                                    max(
                                        m2.read().unwrap()[[i, j, l]],
                                        max(
                                            m3.read().unwrap()[[i, j, l]],
                                            max(
                                                m4.read().unwrap()[[i, j, l]],
                                                max(
                                                    m5.read().unwrap()[[i, j, l]],
                                                    m6.read().unwrap()[[i, j, l]],
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        );
                    }
                }
            }
        });
    })
}
fn sw3d_pluto(
    m1: &mut TSMatrix3d<i32>,
    m2: &mut TSMatrix3d<i32>,
    m3: &mut TSMatrix3d<i32>,
    m4: &mut TSMatrix3d<i32>,
    m5: &mut TSMatrix3d<i32>,
    m6: &mut TSMatrix3d<i32>,
    h: &mut TSMatrix3d<i32>,
    w: &Vec<i32>,
    a: &Vec<char>,
    b: &Vec<char>,
    c: &Vec<char>,
    n: usize,
) {
    println!("pluto\n");
    if n >= 1 {
        for t2 in 0..=floord(n as f64, 8f64) {
            let lbp = max(0, ceild((16 - t2 - n) as f64, 16f64));
            let ubp = min(floord(n as f64, 16f64), t2);
            rayon::broadcast(|_| {
                for t4 in lbp..=ubp {
                    for t6 in 0..=floord(n as f64, 16f64) {
                        for t7 in max(1, 16 * t2 - 16 * t4)..=min(n, 16 * t2 - 16 * t4 + 15) {
                            for t9 in max(1, 16 * t4)..=min(n, 16 * t4 + 15) {
                                for t11 in max(1, 16 * t6)..=min(n, 16 * t6 + 15) {
                                    m1.write().unwrap()[[t7, t9, t11]] = i32::MIN;
                                    for t13 in 1..=t7 {
                                        m1.write().unwrap()[[t7, t9, t11]] = max(
                                            m1.read().unwrap()[[t7, t9, t11]],
                                            h.read().unwrap()[[t7 - t13, t9, t11]] - 2 * w[t13],
                                        );
                                    }
                                    m2.write().unwrap()[[t7, t9, t11]] = i32::MIN;
                                    for t13 in 1..=t9 {
                                        m2.write().unwrap()[[t7, t9, t11]] = max(
                                            m2.read().unwrap()[[t7, t9, t11]],
                                            h.read().unwrap()[[t7, t9 - t13, t11]] - 2 * w[t13],
                                        );
                                    }
                                    m3.write().unwrap()[[t7, t9, t11]] = i32::MIN;
                                    for t13 in 1..=t11 {
                                        m3.write().unwrap()[[t7, t9, t11]] = max(
                                            m3.read().unwrap()[[t7, t9, t11]],
                                            h.read().unwrap()[[t7, t9, t11 - t13]] - 2 * w[t13],
                                        );
                                    }
                                    m4.write().unwrap()[[t7, t9, t11]] = i32::MIN;
                                    for t13 in 1..=min(t7, t9) {
                                        m4.write().unwrap()[[t7, t9, t11]] = max(
                                            m4.read().unwrap()[[t7, t9, t11]],
                                            h.read().unwrap()[[t7 - t13, t9 - t13, t11]] - w[t13]
                                                + s(a[t7], b[t9]),
                                        );
                                    }
                                    m5.write().unwrap()[[t7, t9, t11]] = i32::MIN;
                                    for t13 in 1..=min(t11, t9) {
                                        m5.write().unwrap()[[t7, t9, t11]] = max(
                                            m5.read().unwrap()[[t7, t9, t11]],
                                            h.read().unwrap()[[t7, t9 - t13, t11 - t13]] - w[t13]
                                                + s(b[t9], c[t11]),
                                        );
                                    }
                                    m6.write().unwrap()[[t7, t9, t11]] = i32::MIN;
                                    for t13 in 1..=min(t11, t7) {
                                        m6.write().unwrap()[[t7, t9, t11]] = max(
                                            m6.read().unwrap()[[t7, t9, t11]],
                                            h.read().unwrap()[[t7 - t13, t9, t11 - t13]] - w[t13]
                                                + s(a[t7], c[t11]),
                                        );
                                    }
                                    h.write().unwrap()[[t7, t9, t11]] = max(
                                        0,
                                        max(
                                            h.read().unwrap()[[t7 - 1, t9 - 1, t11 - 1]]
                                                + s(a[t7], b[t9])
                                                + s(a[t7], c[t11])
                                                + s(b[t9], c[t11]),
                                            max(
                                                m1.read().unwrap()[[t7, t9, t11]],
                                                max(
                                                    m2.read().unwrap()[[t7, t9, t11]],
                                                    max(
                                                        m3.read().unwrap()[[t7, t9, t11]],
                                                        max(
                                                            m4.read().unwrap()[[t7, t9, t11]],
                                                            max(
                                                                m5.read().unwrap()[[t7, t9, t11]],
                                                                m6.read().unwrap()[[t7, t9, t11]],
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
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
fn sw_pluto3d(
    m1: &mut TSMatrix3d<i32>,
    m2: &mut TSMatrix3d<i32>,
    m3: &mut TSMatrix3d<i32>,
    m4: &mut TSMatrix3d<i32>,
    m5: &mut TSMatrix3d<i32>,
    m6: &mut TSMatrix3d<i32>,
    h: &mut TSMatrix3d<i32>,
    w: &Vec<i32>,
    a: &Vec<char>,
    b: &Vec<char>,
    c: &Vec<char>,
    n: usize,
) {
    rayon::scope(|x| {
        x.spawn(|_| {
            for i in 1..=n {
                for j in 1..=n {
                    for l in 1..=n {
                        m1.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=i {
                            m1.write().unwrap()[[i, j, l]] = max(
                                m1.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i - k, j, l]] - 2 * w[k],
                            );
                        }
                        m2.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=j {
                            m2.write().unwrap()[[i, j, l]] = max(
                                m2.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i, j - k, l]] - 2 * w[k],
                            );
                        }
                        m3.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=l {
                            m3.write().unwrap()[[i, j, l]] = max(
                                m3.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i, j, l - k]] - 2 * w[k],
                            );
                        }
                        m4.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=min(i, j) {
                            m4.write().unwrap()[[i, j, l]] = max(
                                m4.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i - k, j - k, l]] - w[k] + s(a[i], b[j]),
                            );
                        }
                        m5.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=min(j, l) {
                            m5.write().unwrap()[[i, j, l]] = max(
                                m5.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i, j - k, l - k]] - w[k] + s(b[j], c[l]),
                            );
                        }
                        m6.write().unwrap()[[i, j, l]] = i32::MIN;
                        for k in 1..=min(i, l) {
                            m6.write().unwrap()[[i, j, l]] = max(
                                m6.read().unwrap()[[i, j, l]],
                                h.read().unwrap()[[i - k, j, l - k]] - w[k] + s(a[i], c[l]),
                            );
                        }
                        h.write().unwrap()[[i, j, l]] = max(
                            0,
                            max(
                                h.read().unwrap()[[i - 1, j - 1, l - 1]]
                                    + s(a[i], b[j])
                                    + s(a[i], c[l])
                                    + s(b[j], c[l]),
                                max(
                                    m1.read().unwrap()[[i, j, l]],
                                    max(
                                        m2.read().unwrap()[[i, j, l]],
                                        max(
                                            m3.read().unwrap()[[i, j, l]],
                                            max(
                                                m4.read().unwrap()[[i, j, l]],
                                                max(
                                                    m5.read().unwrap()[[i, j, l]],
                                                    m6.read().unwrap()[[i, j, l]],
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        );
                    }
                }
            }
        });
    })
}
fn sw_traco3d(
    m1: &mut TSMatrix3d<i32>,
    m2: &mut TSMatrix3d<i32>,
    m3: &mut TSMatrix3d<i32>,
    m4: &mut TSMatrix3d<i32>,
    m5: &mut TSMatrix3d<i32>,
    m6: &mut TSMatrix3d<i32>,
    h: &mut TSMatrix3d<i32>,
    w: &Vec<i32>,
    a: &Vec<char>,
    b: &Vec<char>,
    c: &Vec<char>,
    n: usize,
) {
    rayon::broadcast(|_| {
        for c1 in 0..=(n - 1) / 16 {
            for c3 in 0..=(n - 1) / 16 {
                for c5 in 0..=(n - 1) / 16 {
                    for c7 in 16 * c1 + 1..=min(n, 16 * c1 + 16) {
                        for c9 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                            for c11 in 16 * c5 + 1..=min(n, 16 * c5 + 16) {
                                m1.write().unwrap()[[c7, c9, c11]] = i32::MIN;
                                m2.write().unwrap()[[c7, c9, c11]] = i32::MIN;
                                m3.write().unwrap()[[c7, c9, c11]] = i32::MIN;
                                m4.write().unwrap()[[c7, c9, c11]] = i32::MIN;
                                m5.write().unwrap()[[c7, c9, c11]] = i32::MIN;
                                m6.write().unwrap()[[c7, c9, c11]] = i32::MIN;
                            }
                        }
                    }
                }
            }
        }
    });
    for c1 in 0..=floord((n - 1) as f64, 8f64) {
        rayon::broadcast(|_| {
            for c3 in max(0, c1 - (n + 15) / 16 + 1)..min(c1, (n - 1) / 16) {
                for c5 in 0..=(n - 1) / 16 {
                    for c6 in 0..=6 {
                        if c6 == 6 {
                            for c9 in 16 * c1 - 16 * c3 + 1..=min(n, 16 * c1 - 16 * c3 + 16) {
                                for c11 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                                    for c13 in 16 * c5 + 1..=min(n, 16 * c5 + 16) {
                                        for c14 in max(0, 16 * c1 - 16 * c3 - c9 + 2)
                                            ..min(1, (-16 * c3 as i32 + c11 as i32 - 1) as usize)
                                        {
                                            if c14 == 1 {
                                                for c15 in 1..=c11 {
                                                    m2.write().unwrap()[[c9, c11, c13]] = max(
                                                        m2.read().unwrap()[[c9, c11, c13]],
                                                        h.read().unwrap()[[c9, c11 - c15, c13]]
                                                            - 2 * w[c15],
                                                    );
                                                }
                                            } else {
                                                for c15 in 1..=c9 {
                                                    m1.write().unwrap()[[c9, c11, c13]] = max(
                                                        m1.read().unwrap()[[c9, c11, c13]],
                                                        h.read().unwrap()[[c9 - c15, c11, c13]]
                                                            - 2 * w[c15],
                                                    );
                                                }
                                            }
                                        }
                                        for c14 in max(2, 16 * c5 - c13 + 4)
                                            ..=min(
                                                min(
                                                    3,
                                                    (-16 * c1 as i32) as usize + 16 * c3 + c9 + 1,
                                                ),
                                                (-16 * c3 as i32) as usize + c11 + 1,
                                            )
                                        {
                                            if c14 == 3 {
                                                for c15 in 1..=min(c9, c11) {
                                                    m4.write().unwrap()[[c9, c11, c13]] = max(
                                                        m4.read().unwrap()[[c9, c11, c13]],
                                                        h.read().unwrap()
                                                            [[c9 - c15, c11 - c15, c13]]
                                                            - w[c15]
                                                            + s(
                                                                (c9 as u8) as char,
                                                                (c11 as u8) as char,
                                                            ),
                                                    );
                                                }
                                            } else {
                                                for c15 in 1..=c13 {
                                                    m3.write().unwrap()[[c9, c11, c13]] = max(
                                                        m1.read().unwrap()[[c9, c11, c13]],
                                                        h.read().unwrap()[[c9, c11, c13 - c15]]
                                                            - 2 * w[c15],
                                                    );
                                                }
                                            }
                                        }
                                        if c13 >= 16 * c5 + 2 {
                                            for c14 in max(4, 16 * c3 - c11 + 6)
                                                ..=min(
                                                    5,
                                                    (-16 * c1 as i32) as usize + 16 * c3 + c9 + 3,
                                                )
                                            {
                                                if c14 == 5 {
                                                    for c15 in 1..min(c9, c13) {
                                                        m6.write().unwrap()[[c9, c11, c13]] = max(
                                                            m6.read().unwrap()[[c9, c11, c13]],
                                                            h.read().unwrap()
                                                                [[c9 - c15, c11, c13 - c15]]
                                                                - w[c15]
                                                                + s(
                                                                    (c11 as u8) as char,
                                                                    (c13 as u8) as char,
                                                                ),
                                                        );
                                                    }
                                                } else {
                                                    for c15 in 1..min(c11, c13) {
                                                        m5.write().unwrap()[[c9, c11, c13]] = max(
                                                            m5.read().unwrap()[[c9, c11, c13]],
                                                            h.read().unwrap()
                                                                [[c9, c11 - c15, c13 - c15]]
                                                                - w[c15]
                                                                + s(
                                                                    (c11 as u8) as char,
                                                                    (c13 as u8) as char,
                                                                ),
                                                        );
                                                    }
                                                }
                                                h.write().unwrap()[[c9, c11, c13]] = max(
                                                    0,
                                                    max(
                                                        h.read().unwrap()
                                                            [[c9 - 1, c11 - 1, c13 - 1]]
                                                            + s(a[c9], b[c11])
                                                            + s(a[c9], c[c13])
                                                            + s(b[c11], c[c13]),
                                                        max(
                                                            m1.read().unwrap()[[c9, c11, c13]],
                                                            max(
                                                                m2.read().unwrap()[[c9, c11, c13]],
                                                                max(
                                                                    m3.read().unwrap()
                                                                        [[c9, c11, c13]],
                                                                    max(
                                                                        m4.read().unwrap()
                                                                            [[c9, c11, c13]],
                                                                        max(
                                                                            m5.read().unwrap()
                                                                                [[c9, c11, c13]],
                                                                            m6.read().unwrap()
                                                                                [[c9, c11, c13]],
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        } else if c6 == 5 {
                            for c7 in 0..=min(c1 - c3, c5) {
                                for c9 in 16 * c1 - 16 * c3 + 1..=min(n, 16 * c1 - 16 * c3 + 16) {
                                    for c11 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                                        if 16 * c3 + c9 >= 16 * c1 + 2 {
                                            for c15 in 16 * c7 + 1
                                                ..=min(min(16 * c5 + 1, 16 * c7 + 16), c9)
                                            {
                                                m6.write().unwrap()[[c9, c11, 16 * c5 + 1]] = max(
                                                    m6.read().unwrap()[[c9, c11, 16 * c5 + 1]],
                                                    h.read().unwrap()
                                                        [[c9, c11 - c15, (16 * c5 + 1) - c15]]
                                                        - w[c15]
                                                        + s(
                                                            (c9 as u8) as char,
                                                            ((16 * c5 + 1) as u8) as char,
                                                        ),
                                                );
                                            }
                                        } else {
                                            for c13 in 16 * c5 + 1..=min(n, 16 * c5 + 16) {
                                                for c15 in 16 * c7 + 1
                                                    ..=min(
                                                        min(16 * c1 - 16 * c3 + 1, 16 * c7 + 16),
                                                        c13,
                                                    )
                                                {
                                                    m6.write().unwrap()
                                                        [[16 * c1 - 16 * c3 + 1, c11, c13]] = max(
                                                        m6.write().unwrap()
                                                            [[16 * c1 - 16 * c3 + 1, c11, c13]],
                                                        h.read().unwrap()[[
                                                            (16 * c1 - 16 * c3 + 1) - c15,
                                                            c11,
                                                            c13 - c15,
                                                        ]] - w[c15]
                                                            + s(
                                                                ((16 * c1 - 16 * c3 + 1) as u8)
                                                                    as char,
                                                                (c13 as u8) as char,
                                                            ),
                                                    )
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        //do later
                        else if c6 == 4 {
                            for c7 in 0..min(c3, c5) {
                                for c9 in 16 * c1 - 16 * c3 + 1..=min(n, 16 * c1 - 16 * c3 + 16) {
                                    for c11 in 16 * c3 + 1..min(n, 16 * c3 + 16) {
                                        if c11 >= 16 * c3 + 2 {
                                            for c15 in 16 * c7 + 1
                                                ..=min(min(16 * c5 + 1, 16 * c7 + 16), c11)
                                            {
                                                m5.write().unwrap()[[c9, c11, 16 * c5 + 1]] = max(
                                                    m5.read().unwrap()[[c9, c11, 16 * c5 + 1]],
                                                    h.read().unwrap()
                                                        [[c9, c11 - c15, (16 * c5 + 1) - c15]]
                                                        - w[c15]
                                                        + s(
                                                            (c11 as u8) as char,
                                                            ((16 * c3 + 1) as u8) as char,
                                                        ),
                                                );
                                            }
                                        } else {
                                            for c13 in 16 * c5 + 1..=min(n, 16 * c5 + 16) {
                                                for c15 in 16 * c7 + 1
                                                    ..=min(min(16 * c3 + 1, 16 * c7 + 16), c13)
                                                {
                                                    m5.write().unwrap()[[c9, 16 * c3 + 1, c13]] =
                                                        max(
                                                            m5.read().unwrap()
                                                                [[c9, 16 * c3 + 1, c13]],
                                                            h.read().unwrap()
                                                                [[c9, (16 * c3 + 1) - c15, c13]]
                                                                - w[c15]
                                                                + s(
                                                                    (c9 as u8) as char,
                                                                    ((16 * c3 + 1) as u8) as char,
                                                                ),
                                                        );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if c6 == 3 {
                            for c7 in 0..=min(c3, c1 - c3) {
                                for c9 in 16 * c1 - 16 * c3 + 1..=min(n, 16 * c1 - 16 * c3 + 16) {
                                    if 16 * c3 + c9 >= 16 * c1 + 2 {
                                        for c13 in 16 * c5 + 1..=min(n, 16 * c5 + 16) {
                                            for c15 in 16 * c7 + 1
                                                ..=min(min(16 * c3 + 1, 16 * c7 + 16), c9)
                                            {
                                                m4.write().unwrap()[[c9, 16 * c3 + 1, c13]] = max(
                                                    m4.read().unwrap()[[c9, 16 * c3 + 1, c13]],
                                                    h.read().unwrap()
                                                        [[c9 - c15, (16 * c3 + 1) - c15, c13]]
                                                        - w[c15]
                                                        + s(
                                                            (c9 as u8) as char,
                                                            ((16 * c3 + 1) as u8) as char,
                                                        ),
                                                );
                                            }
                                        }
                                    } else {
                                        for c11 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                                            for c13 in 16 * c5 + 1..=min(n, 16 * c3 + 16) {
                                                for c15 in 16 * c7 + 1
                                                    ..=min(
                                                        min(16 * c1 - 16 * c3 + 1, 16 * c7 + 16),
                                                        c11,
                                                    )
                                                {
                                                    m4.write().unwrap()
                                                        [[16 * c1 - 16 * c3 + 1, c11, c13]] = max(
                                                        m4.read().unwrap()
                                                            [[16 * c1 - 16 * c3 + 1, c11, c13]],
                                                        h.read().unwrap()[[
                                                            (16 * c1 - 16 * c3 + 1) - c15,
                                                            c11 - c15,
                                                            c13,
                                                        ]] - w[c15]
                                                            + s(
                                                                ((16 * c1 - 16 * c3 + 1) as u8)
                                                                    as char,
                                                                (c11 as u8) as char,
                                                            ),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if c6 == 2 {
                            for c7 in 0..=c5 {
                                for c9 in 16 * c1 - 16 * c3 + 1..=min(n, 16 * c1 - 16 * c3 + 16) {
                                    for c11 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                                        for c15 in 16 * c7 + 1..=min(16 * c5 + 1, 16 * c7 + 16) {
                                            m3.write().unwrap()[[c9, c11, 16 * c5 + 1]] = max(
                                                m3.read().unwrap()[[c9, c11, (16 * c5 + 1)]],
                                                h.read().unwrap()[[c9, c11, (16 * c5 + 1) - c15]]
                                                    - 2 * w[c15],
                                            );
                                        }
                                    }
                                }
                            }
                        } else if c6 == 1 {
                            for c7 in 0..=c5 {
                                for c9 in 16 * c1 - 16 * c3 + 1..=min(n, 16 * c1 - 16 * c3 + 16) {
                                    for c13 in 16 * c5 + 1..=min(n, 16 * c5 + 16) {
                                        for c15 in 16 * c7 + 1..=min(16 * c5 + 1, 16 * c7 + 16) {
                                            m2.write().unwrap()[[c9, 16 * c3 + 1, c13]] = max(
                                                m2.read().unwrap()[[c9, (16 * c3 + 1), c13]],
                                                h.read().unwrap()[[c9, (16 * c5 + 1) - c15, c13]]
                                                    - 2 * w[c15],
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            for c7 in 0..=c1 - c3 {
                                for c11 in 16 * c3 + 1..=min(n, 16 * c3 + 16) {
                                    for c13 in 16 * c5 + 1..=min(n, 16 * c5 + 16) {
                                        for c15 in
                                            16 * c7 + 1..=min(16 * c1 - 16 * c3 + 1, 16 * c7 + 16)
                                        {
                                            m1.write().unwrap()
                                                [[16 * c1 - 16 * c3 + 1, c11, c13]] = max(
                                                m2.read().unwrap()
                                                    [[16 * c1 - 16 * c3 + 1, c11, c13]],
                                                h.read().unwrap()
                                                    [[(16 * c1 - 16 * c3 + 1) - c15, c11, c13]]
                                                    - 2 * w[c15],
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
pub fn sw_tstile(
    m1: &mut TSMatrix3d<i32>,
    m2: &mut TSMatrix3d<i32>,
    m3: &mut TSMatrix3d<i32>,
    m4: &mut TSMatrix3d<i32>,
    m5: &mut TSMatrix3d<i32>,
    m6: &mut TSMatrix3d<i32>,
    h: &mut TSMatrix3d<i32>,
    w: &Vec<i32>,
    a: &Vec<char>,
    b: &Vec<char>,
    c: &Vec<char>,
    n: usize,
) {
    for c0 in 0..n + floord((n - 1) as f64, 8f64) {
        rayon::broadcast(|_| {
            for c1 in max(0, c0 - (n + 7) / 8 + 1)..=min(n - 1, c0) {
                for c2 in max(0, c0 - c1 - (n + 15) / 16 + 1)..=min(c0 - c1, (n - 1) / 16) {
                    for c4 in 16 * c0 - 16 * c1 + 2
                        ..=min(
                            min(
                                min(min(2 * n, 16 * c0 - 15 * c1 + 2), 16 * c0 - 16 * c1 + 32),
                                n + 16 * c2 + 16,
                            ),
                            n + 16 * c0 - 16 * c1 - 16 * c2 + 16,
                        )
                    {
                        for c9 in max(
                            max(
                                16 * c2 + 1,
                                (-16 as i32 * c0 as i32) as usize + 16 * c1 + 16 * c2 + c4 - 16,
                            ),
                            (-(n as i32) + c4 as i32) as usize,
                        )
                            ..=min(
                                min(n, 16 * c2 + 16),
                                (-16 * c0 as i32) as usize + 16 * c1 + 16 * c2 + c4 - 1,
                            )
                        {
                            m1.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                            m2.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                            m3.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                            m6.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                        }
                        for c4 in 16 * c0 - 15 * c1 + 3
                            ..=min(
                                min(
                                    min(
                                        min(2 * n + c1 + 1, 16 * c0 - 15 * c1 + 33),
                                        16 * c0 - 15 * c1 + 16 * c2 + 3,
                                    ),
                                    n + c1 + 16 * c2 + 17,
                                ),
                                n + 16 * c0 - 15 * c1 - 16 * c2 + 17,
                            )
                        {
                            if 16 * c0 >= 17 * c1 + 16 * c2 + 2 {
                                for c9 in max(
                                    max(
                                        16 * c2 + 1,
                                        (-16 * c0 as i32) as usize + 16 * c1 + 16 * c2 + c4 - 16,
                                    ),
                                    (-(n as i32) + c4 as i32) as usize,
                                )
                                    ..=min(
                                        min(n, 16 * c2 + 16),
                                        (-16 * c0 as i32) as usize + 16 * c1 + 16 * c2 + c4 - 1,
                                    )
                                {
                                    m1.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                    m2.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                    m3.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                    m6.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                }
                            }
                            for c5 in max(
                                max(
                                    c0 - c1 - c2,
                                    floord((-(n as i32) - c1 as i32 + c4 as i32 - 1) as f64, 16f64),
                                ),
                                (-(c2 as i32) as usize
                                    + (-(c1 as i32) as usize + c4 - 1) as usize / 16
                                    - 1),
                            )
                                ..=min(
                                    n / 16,
                                    (-(c2 as i32)) as usize + ((-(c1 as i32)) as usize) + c4 - 2,
                                )
                            {
                                for c9 in max(
                                    max(
                                        max(
                                            16 * c2 + 1,
                                            (-16 * c0 as i32) as usize + 15 * c1 + 16 * c2 + c4
                                                - 17,
                                        ),
                                        -(n as i32) as usize - c1 + c4 - 1,
                                    ),
                                    -(c1 as i32) as usize + c4 - 16 * c5 - 16,
                                )
                                    ..=min(
                                        min(
                                            min(n, 16 * c2 + 16),
                                            (-16 * c0 as i32) as usize + 15 * c1 + 16 * c2 + c4 - 2,
                                        ),
                                        -(c1 as i32) as usize + c4 - 16 * c5 - 1,
                                    )
                                {
                                    m4.write().unwrap()
                                        [[c1 + 1, c9, -(c1 as i32) as usize + c4 - c9 - 1]] =
                                        i32::MIN;
                                    if c1 + c2 == c0 && c5 == 0 && n + c9 >= c4 && c9 + 16 >= c4 {
                                        m1.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                        m2.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                        m3.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                        m6.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                    }
                                }
                                if c1 >= 15 && c1 + c2 == c0 {
                                    for c9 in c4 - 16..=min(n, 16 * c0 - 16 * c1 + 16) {
                                        m1.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                        m2.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                        m3.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                        m6.write().unwrap()[[c1 + 1, c9, c4 - c9]] = i32::MIN;
                                    }
                                }
                            }
                        }
                        for c4 in 16 * c0 - 15 * c1 + 16 * c2 + 4
                            ..=min(
                                min(
                                    min(
                                        min(3 * n + c1 + 1, 2 * n + 16 * c0 - 15 * c1 + 2),
                                        2 * n + 16 * c0 - 15 * c1 - 16 * c2 + 17,
                                    ),
                                    n + c1 + 32 * c2 + 33,
                                ),
                                16 * c0 - 15 * c1 + 16 * c2 + 49,
                            )
                        {
                            for c5 in max(
                                max(
                                    c0 - c1 - c2,
                                    floord(((-2 * n as i32) as usize - c1 + c4 - 1) as f64, 16f64),
                                ),
                                (-2 * c2 as i32) as usize + (-(c1 as i32) as usize + c4 - 1) / 16
                                    - 2,
                            )
                                ..=min(
                                    min(c0 - c1 - c2 + 1, (c1 + 1) / 16 - 1),
                                    (-2 * c2 as i32) as usize
                                        + (-(c1 as i32) as usize + c4 - 3) / 16,
                                )
                            {
                                for c9 in max(
                                    max(
                                        16 * c2 + 1,
                                        (-8 * c0 as i32) as usize + 7 * c1 + 8 * c2 + (c1 + c4) / 2
                                            - 8,
                                    ),
                                    -(c1 as i32) as usize - 8 * c5 + (c1 + c4 + 1) / 2 - 8,
                                )
                                    ..=min(
                                        min(
                                            min(n, 16 * c2 + 16),
                                            -(8 as i32) as usize * c0
                                                + 7 * c1
                                                + 8 * c2
                                                + (c1 + c4) / 2
                                                - 1,
                                        ),
                                        -(c1 as i32) as usize - 8 * c5 + (c1 + c4 + 1) / 2 - 1,
                                    )
                                {
                                    m5.write().unwrap()
                                        [[c1 + 1, c9, -(c1 as i32) as usize + c4 - 2 * c9 - 1]] =
                                        i32::MIN;
                                    if c2 == 0 && c1 + c5 == c0 && 16 * c0 + c9 + 16 >= 15 * c1 + c4
                                    {
                                        m4.write().unwrap()
                                            [[c1 + 1, c9, (-(c1 as i32) as usize + c4 - c9 - 1)]] =
                                            i32::MIN;
                                    }
                                }
                                if c2 == 0 && 16 * c0 + 32 >= 15 * c1 + c4 && c1 + c5 == c0 + 1 {
                                    m4.write().unwrap()[[
                                        c1 + 1,
                                        (-16 * c0 as i32) as usize + 15 * c1 + c4 - 17,
                                        16 * c0 - 16 * c1 + 16,
                                    ]] = i32::MIN;
                                }
                                for c9 in max(
                                    max(
                                        (-16 * c0 as i32) as usize + 15 * c1 + 16 * c2 + c4 - 17,
                                        -(c1 as i32) as usize + c4 - 16 * c5 - 16,
                                    ),
                                    (-8 * c0 as i32) as usize + 7 * c1 + 8 * c2 + (c1 + c4) / 2,
                                )
                                    ..=min(
                                        min(
                                            min(n, 16 * c2 + 16),
                                            (-16 * c0 as i32) as usize + 15 * c1 + 16 * c2 + c4 - 2,
                                        ),
                                        -(c1 as i32) as usize + c4 - 16 * c5 - 1,
                                    )
                                {
                                    //64 line
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
