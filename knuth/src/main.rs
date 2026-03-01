//DO WHOLE REWRITE OF GETTERS AND SETTERS
use array2d::{Array2D, Error};
use rand::prelude::*;
use rayon::ThreadPool;
use rayon::ThreadPoolBuildError;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::env;
use std::thread;
use std::time::{Duration, Instant};
mod mem;
fn index_correction(num: i32, correction: usize) -> usize {
    if num < 0 {
        return (correction as i32 + num) as usize;
    }
    return num as usize;
}
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn main() {
    let mut num_proc = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(value) => num_proc = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    let mut kind = 1;
    let mut n: usize = 8;
    let mut dim: usize = 12;
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
    let mut c = mem::mem_lock(dim);
    let mut ck = mem::mem(dim);
    let mut w = mem::mem(dim);
    for i in 0..dim {
        for j in 0..dim {
            c.write().unwrap()[(i, j)] = (i + j) as i32;
            c.write().unwrap()[(i, j)] = ck[(i, j)];
            ck[(i, j)] = i as i32 - j as i32;
        }
    }
    let check: bool = true;
    let start = Instant::now();
    if kind == 1 || check == true {
        rayon::scope(|s| {
            s.spawn(|_| {
                for i in n - 1..=1 {
                    for j in i + 1..=n {
                        for k in i + 1..j {
                            ck[(i, j)] = min(ck[(i, j)], w[(i, j)] + ck[(i, k)] + ck[(k, j)]);
                            /*
                            ck.set(
                                i,
                                j,
                                min(
                                    *ck.get(i, j).unwrap(),
                                    w.get(i, j).unwrap()
                                        + ck.get(i, k).unwrap()
                                        + ck.get(k, j).unwrap(),
                                ),
                            )
                            .unwrap();
                            */
                        }
                    }
                }
            });
        })
    }
    if kind == 2 {
        if n >= 3 {
            for t2 in -1..(f64::floor(((n - 16) as f64) / 16f64)) as i32 {
                let lbp = t2 + 1;
                let ubp = min(
                    floord(n as f64, 16f64),
                    floord((16 * t2 + n as i32 + 13) as f64, 16f64),
                    //f64::floor(n as f64 / 16f64) as i32,
                    //(f64::floor((16 * t2 + n as i32 + 13) as f64 / 16f64)) as i32,
                ) as i32;
                rayon::broadcast(|_| {
                    for t4 in lbp..=ubp {
                        for t5 in max(max(-(n as i32) + 2, 16 * t2 - 16 * t4), -16 * t4 - 13)
                            ..=(16 * t2 - 16 * t4 + 15)
                        {
                            for t7 in max(16 * t4, -(t5 as i32) + 2)..=min(n as i32, 16 * t4 + 15) {
                                for t9 in t5 + 1..=t7 - 1 {
                                    c.write()
                                        .unwrap()
                                        .set(
                                            (c.read().unwrap().row_len() - (t5 as usize)),
                                            t7 as usize,
                                            min(
                                                *c.read()
                                                    .unwrap()
                                                    .get(
                                                        c.read().unwrap().row_len() - (t5 as usize),
                                                        t7 as usize,
                                                    )
                                                    .unwrap(),
                                                w.get(w.row_len() - (t5 as usize), t7 as usize)
                                                    .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(
                                                            c.read().unwrap().row_len()
                                                                - (t5 as usize),
                                                            t9 as usize,
                                                        )
                                                        .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(t9 as usize, t7 as usize)
                                                        .unwrap(),
                                            ),
                                        )
                                        .unwrap()
                                }
                            }
                        }
                    }
                });
            }
        }
    }
    if kind == 3 {
        for c1 in 0..(n as i32) + (f64::floor((-3 * (n as i32) - 3) as f64 / 8f64)) as i32 {
            rayon::broadcast(|_| {
                for c3 in max(0, c1 - (n as i32 + 6) / 8 + 1)
                    ..=min(n as i32 / 2 - 1, c1 - (c1 + 6) / 5 + 1)
                {
                    for c5 in 0..c3 / 128 {
                        for c7 in max(
                            max(-(n as i32) + 2 * c3 + 1, -(n as i32) + 8 * c1 - 8 * c3 + 1),
                            -(n as i32) + c3 + 128 * c5 + 2,
                        )
                            ..=min(-1, -(n as i32) + 8 * c1 - 8 * c3 + 8)
                        {
                            if n + 8 * (c3 as usize) + (c7 as usize) >= 8 * (c1 as usize) + 2 {
                                for c11 in 256 * c5 - c7 + 1..=min(2 * c3 - c7, 256 * c5 - c7 + 256)
                                {
                                    c.write()
                                        .unwrap()
                                        .set(
                                            c.read().unwrap().row_len() - (c7 as usize),
                                            2usize * (c3 as usize) - (c7 as usize) + 1,
                                            min(
                                                *c.read()
                                                    .unwrap()
                                                    .get(
                                                        c.read().unwrap().row_len() - (c7 as usize),
                                                        (2 * c3 - c7 + 2) as usize,
                                                    )
                                                    .unwrap(),
                                                *w.get(
                                                    w.row_len() - (c7 as usize),
                                                    2 * (c3 as usize) - (c7 as usize) + 2,
                                                )
                                                .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(
                                                            c.read().unwrap().row_len()
                                                                - (c7 as usize),
                                                            c11 as usize,
                                                        )
                                                        .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(
                                                            c11 as usize,
                                                            2 * (c3 as usize) - (c7 as usize) + 2,
                                                        )
                                                        .unwrap(),
                                            ),
                                        )
                                        .unwrap();
                                    if 128 * c5 + 128 >= c3
                                        && n + (c7 as usize) >= 2 * (c3 as usize) + 2
                                    {
                                        if c3 >= 128 * c5 + 1 {
                                            for c11 in -(c7 as i32) + 1..=256 * c5 - c7 {
                                                c.write()
                                                    .unwrap()
                                                    .set(
                                                        c.read().unwrap().row_len() - (c7 as usize),
                                                        2 * (c3 as usize) - (c7 as usize) + 2,
                                                        min(
                                                            *c.read()
                                                                .unwrap()
                                                                .get(
                                                                    c.read().unwrap().row_len()
                                                                        - (c7 as usize),
                                                                    2 * (c3 as usize)
                                                                        - (c7 as usize)
                                                                        + 2,
                                                                )
                                                                .unwrap(),
                                                            *w.get(
                                                                c.read().unwrap().row_len()
                                                                    - (c7 as usize),
                                                                2 * (c3 as usize) - (c7 as usize)
                                                                    + 2,
                                                            )
                                                            .unwrap()
                                                                + c.read()
                                                                    .unwrap()
                                                                    .get(
                                                                        c.read().unwrap().row_len()
                                                                            - (c7 as usize),
                                                                        c11 as usize,
                                                                    )
                                                                    .unwrap()
                                                                + c.read()
                                                                    .unwrap()
                                                                    .get(
                                                                        c11 as usize,
                                                                        2 * (c3 as usize)
                                                                            - (c7 as usize)
                                                                            + 2,
                                                                    )
                                                                    .unwrap(),
                                                        ),
                                                    )
                                                    .unwrap()
                                            }
                                            for c11 in 256 * c5 - c7 + 1
                                                ..=min(2 * c3 - c7 + 1, 256 * c5 - c7 + 256)
                                            {
                                                c.write()
                                                    .unwrap()
                                                    .set(
                                                        c.read().unwrap().row_len() - (c7 as usize),
                                                        2 * (c3 as usize) - (c7 as usize) + 2,
                                                        min(
                                                            *c.read()
                                                                .unwrap()
                                                                .get(
                                                                    c.read().unwrap().row_len()
                                                                        - (c7 as usize),
                                                                    2 * (c3 as usize)
                                                                        - (c7 as usize)
                                                                        + 2,
                                                                )
                                                                .unwrap(),
                                                            w.get(
                                                                c.read().unwrap().row_len()
                                                                    - (c7 as usize),
                                                                2 * (c3 as usize) - (c7 as usize)
                                                                    + 2,
                                                            )
                                                            .unwrap()
                                                                + c.read()
                                                                    .unwrap()
                                                                    .get(
                                                                        c.read().unwrap().row_len()
                                                                            - (c7 as usize),
                                                                        c11 as usize,
                                                                    )
                                                                    .unwrap()
                                                                + c.read()
                                                                    .unwrap()
                                                                    .get(
                                                                        c11 as usize,
                                                                        2 * (c3 as usize)
                                                                            - (c7 as usize)
                                                                            + 2,
                                                                    )
                                                                    .unwrap(),
                                                        ),
                                                    )
                                                    .unwrap()
                                            }
                                        }
                                    } else {
                                        for c9 in max(
                                            (n as i32) - 8 * c1 + 10 * c3,
                                            (n as i32) - 8 * c1 + 8 * c3 + 256 * c5 + 1,
                                        )
                                            ..=min(n as i32, (n as i32) - 8 * c1 + 10 * c1 + 1)
                                        {
                                            for c11 in (n as i32) - 8 * c1 + 8 * c3 + 256 * c5
                                                ..=min(
                                                    (n as i32) - 8 * c1 + 8 * c3 + 256 * c5 + 255,
                                                    c9 - 1,
                                                )
                                            {
                                                c.write()
                                                    .unwrap()
                                                    .set(
                                                        n - 8 * (c1 as usize) + 8 * (c3 as usize)
                                                            - 1,
                                                        c9 as usize,
                                                        min(
                                                            *c.read()
                                                                .unwrap()
                                                                .get(
                                                                    n - 8 * (c1 as usize)
                                                                        + 8 * (c3 as usize)
                                                                        - 1,
                                                                    c9 as usize,
                                                                )
                                                                .unwrap(),
                                                            w.get(
                                                                n - 8 * (c1 as usize)
                                                                    + 8 * (c3 as usize)
                                                                    - 1,
                                                                c9 as usize,
                                                            )
                                                            .unwrap()
                                                                + c.read()
                                                                    .unwrap()
                                                                    .get(
                                                                        n - 8 * (c1 as usize)
                                                                            + 8 * (c3 as usize)
                                                                            - 1,
                                                                        c11 as usize,
                                                                    )
                                                                    .unwrap()
                                                                + c.read()
                                                                    .unwrap()
                                                                    .get(c11 as usize, c9 as usize)
                                                                    .unwrap(),
                                                        ),
                                                    )
                                                    .unwrap()
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
        if (n - 2) % 8 == 0 {
            for c5 in 0..f64::floor((n - 10) as f64 / 256f64) as i32 {
                for c11 in 256 * c5 + 2..=min((n - 1) as i32, 256 * c5 + 257) {
                    c.write()
                        .unwrap()
                        .set(
                            1,
                            n,
                            min(
                                *c.write().unwrap().get(1, n).unwrap(),
                                w.get(1, n).unwrap()
                                    + c.read().unwrap().get(1, c11 as usize).unwrap()
                                    + c.read().unwrap().get(c11 as usize, n).unwrap(),
                            ),
                        )
                        .unwrap()
                }
            }
        }
    }
    if kind == 4 {
        for c0 in 0..f64::floor((n - 2) as f64 / 8f64) as i32 {
            rayon::broadcast(|_| {
                for c1 in (c0 + 1) / 2..=min(c0, (n - 2) as i32 / 16) {
                    for c3 in
                        max(2, 16 * c0 - 16 * c1 + 1)..=min((n as i32) - 1, 16 * c0 - -16 * c1 + 16)
                    {
                        for c4 in max(0, -c1 + (n as i32 + 1) / 16 - 1)
                            ..=min(((n as i32) - 1) / 16, -c1 + (n as i32 + c3 - 2) / 16)
                        {
                            for c6 in max(
                                max(-(n as i32) + 16 * c1 + 1, -(n as i32) + c3),
                                -16 * c4 - 14,
                            )
                                ..=min(min(-1, -(n as i32) + 16 * c1 + 16), c3 - 16 * c4 - 1)
                            {
                                for c10 in max(16 * c4, -c6 + 1)..=min(16 * c4 + 15, c3 - c6 - 1) {
                                    c.write()
                                        .unwrap()
                                        .set(
                                            c.read().unwrap().row_len() - (c6 as usize),
                                            (c3 - c6) as usize,
                                            min(
                                                *c.read()
                                                    .unwrap()
                                                    .get(
                                                        c.read().unwrap().row_len() - (c6 as usize),
                                                        (c3 - c6) as usize,
                                                    )
                                                    .unwrap(),
                                                w.get(
                                                    c.read().unwrap().row_len() - (c6 as usize),
                                                    (c3 - c6) as usize,
                                                )
                                                .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(
                                                            c.read().unwrap().row_len()
                                                                - (c6 as usize),
                                                            c10 as usize,
                                                        )
                                                        .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(c10 as usize, (c3 - c6) as usize)
                                                        .unwrap(),
                                            ),
                                        )
                                        .unwrap()
                                }
                            }
                        }
                    }
                }
            });
        }
    }
    let end = start.elapsed().as_secs_f64();
    //FIX VALIDITY IN THIS FILE
    println!("The execution took {} seconds", end);
    if check == true {
        for i in 0..dim {
            for j in 0..dim {
                if c.read().unwrap().get(i, j).unwrap() != ck.get(i, j).unwrap() {
                    println!(
                        "err:{} {} {} {}",
                        i,
                        j,
                        c.read().unwrap().get(i, j).unwrap(),
                        ck.get(i, j).unwrap()
                    )
                }
            }
        }
    }
}
