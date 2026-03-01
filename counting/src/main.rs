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
fn paired(rna: &Vec<char>, i: usize, j: usize) -> bool {
    let nt1 = rna[i];
    let nt2 = rna[j];
    match (nt1, nt2) {
        ('A', 'U') => return true,
        ('U', 'A') => return true,
        ('G', 'C') => return true,
        ('C', 'G') => return true,
        ('G', 'U') => return true,
        ('U', 'G') => return true,
        _ => return false,
    }
}
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn main() {
    let mut num_proc: usize = 4;
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
        dim = n + 10;
    }
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
    for i in 0..dim {
        for j in 0..dim {
            c.write()
                .unwrap()
                .set(i, j, (i + j).to_string().trim().parse::<i32>().unwrap())
                .unwrap();
            ck.set(i, j, (i + j).to_string().trim().parse::<i32>().unwrap())
                .unwrap();
        }
    }
    let mut rna: Vec<char> = Vec::new();
    mem::rand_seq(&mut rna, n);
    let check = 1;
    let start = Instant::now();
    if kind == 1 || check == 1 {
        rayon::scope(|s| {
            s.spawn(|_| {
                for i in n - 2..=1 {
                    for j in i + 2..n {
                        for k in i..=j - 1 {
                            let l = if paired(&rna, k, j) {
                                ck.get(i, k - 1).unwrap() + ck.get(k + 1, j - 1).unwrap()
                            } else {
                                0
                            };
                            ck.set(i, j, ck.get(i, j).unwrap() + l).unwrap();
                            ck.set(i, j, ck.get(i, j).unwrap() + ck.get(i, j - 1).unwrap())
                                .unwrap();
                        }
                    }
                }
            });
        });
    }
    if kind == 2 {
        for t1 in 3..=n.to_string().trim().parse::<i32>().unwrap() {
            let lbp = 0;
            //you can also round it, then parse will go through
            let ubp = (f64::floor(f64::from(t1 - 2)) / f64::from(32)) as i32;
            thread_build.as_ref().unwrap().broadcast(|_| {
                for t2 in lbp..=ubp {
                    for t3 in t2..(f64::floor(f64::from(t1)) / f64::from(32)) as i32 {
                        if (t1 >= 32 * t3 + 1) && (t1 <= 32 * t3 + 31) {
                            for t4 in max(1, 32 * t2)..=min(t1 - 2, 32 * t2 + 31) {
                                for t5 in max(32 * t3, t4)..=t1 - 1 {
                                    let l = if paired(
                                        &rna,
                                        t5.to_string().trim().parse::<usize>().unwrap(),
                                        t1.to_string().trim().parse::<usize>().unwrap(),
                                    ) {
                                        c.read()
                                            .unwrap()
                                            .get(
                                                t4.to_string().trim().parse::<usize>().unwrap(),
                                                (t5 - 1)
                                                    .to_string()
                                                    .trim()
                                                    .parse::<usize>()
                                                    .unwrap(),
                                            )
                                            .unwrap()
                                            + c.read()
                                                .unwrap()
                                                .get(
                                                    (t5 + 1)
                                                        .to_string()
                                                        .trim()
                                                        .parse::<usize>()
                                                        .unwrap(),
                                                    (t1 - 1)
                                                        .to_string()
                                                        .trim()
                                                        .parse::<usize>()
                                                        .unwrap(),
                                                )
                                                .unwrap()
                                    } else {
                                        0
                                    };
                                    c.write()
                                        .unwrap()
                                        .set(
                                            t4.to_string().trim().parse::<usize>().unwrap(),
                                            t1.to_string().trim().parse::<usize>().unwrap(),
                                            c.read()
                                                .unwrap()
                                                .get(
                                                    t4.to_string().trim().parse::<usize>().unwrap(),
                                                    t1.to_string().trim().parse::<usize>().unwrap(),
                                                )
                                                .unwrap()
                                                + l,
                                        )
                                        .unwrap();
                                    c.write()
                                        .unwrap()
                                        .set(
                                            t4.to_string().trim().parse::<usize>().unwrap(),
                                            t1.to_string().trim().parse::<usize>().unwrap(),
                                            c.read()
                                                .unwrap()
                                                .get(
                                                    t4.to_string().trim().parse::<usize>().unwrap(),
                                                    t1.to_string().trim().parse::<usize>().unwrap(),
                                                )
                                                .unwrap()
                                                + c.read()
                                                    .unwrap()
                                                    .get(
                                                        t4.to_string()
                                                            .trim()
                                                            .parse::<usize>()
                                                            .unwrap(),
                                                        (t1 - 1)
                                                            .to_string()
                                                            .trim()
                                                            .parse::<usize>()
                                                            .unwrap(),
                                                    )
                                                    .unwrap(),
                                        )
                                        .unwrap();
                                }
                            }
                        }
                        if t1 >= 32 * t3 + 32 {
                            for t4 in max(1, 32 * t2)..=min(t1 - 2, 32 * t2 + 31) {
                                for t5 in max(32 * t3, t4)..=32 * t3 + 31 {
                                    let l = if paired(&rna, t5 as usize, t1 as usize) {
                                        ck.get(t4 as usize, (t5 - 1) as usize).unwrap()
                                            + ck.get((t5 + 1) as usize, (t1 - 1) as usize).unwrap()
                                    } else {
                                        0
                                    };
                                    c.write()
                                        .unwrap()
                                        .set(
                                            t4 as usize,
                                            t1 as usize,
                                            c.write()
                                                .unwrap()
                                                .get(t4 as usize, t1 as usize)
                                                .unwrap()
                                                + l,
                                        )
                                        .unwrap()
                                }
                            }
                        }
                        if t1 == 32 * t3 {
                            for t4 in max(1, 32 * t2)..=min(t1 - 2, 32 * t2 + 31) {
                                if t1 % 32 == 0 {
                                    c.write()
                                        .unwrap()
                                        .set(
                                            t4 as usize,
                                            t1 as usize,
                                            c.read()
                                                .unwrap()
                                                .get(t4 as usize, t1 as usize)
                                                .unwrap()
                                                + c.read()
                                                    .unwrap()
                                                    .get(t4 as usize, (t1 - 1) as usize)
                                                    .unwrap(),
                                        )
                                        .unwrap();
                                }
                            }
                        }
                    }
                }
            });
        }
    }
    if kind == 3 {
        for c1 in 0..n + floord((n - 3) as f64, 128f64) - 2 {
            thread_build.as_ref().unwrap().broadcast(|_| {
                for c3 in max(0, -(n as i32) + (c1 as i32) + 3i32)..=c1 as i32 {
                    for c4 in 0..=1 {
                        if c4 == 1 {
                            for c9 in (n - c1 + 129 * (c3 as usize))
                                ..=min(n, n - c1 + 129 * (c3 as usize) + 127)
                            {
                                for c10 in max(
                                    0,
                                    -(c1 as i32) + 64i32 * c3 - (c9 as i32)
                                        + (n as i32 + (c1 as i32) + c3 + (c9 as i32) + 1) / 2
                                        + 1,
                                )..=1
                                {
                                    if c10 == 1 {
                                        c.write()
                                            .unwrap()
                                            .set(
                                                n - c1 + (c3 as usize) - 2,
                                                c9,
                                                c.read()
                                                    .unwrap()
                                                    .get(
                                                        (n - c1) as usize + c3 as usize
                                                            - 2 as usize,
                                                        c9,
                                                    )
                                                    .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(
                                                            n - c1 + c3 as usize - 2 as usize,
                                                            c9 - 1,
                                                        )
                                                        .unwrap(),
                                            )
                                            .unwrap();
                                    } else {
                                        for c11 in n - c1 + 129 * (c3 as usize) + 1..c9 {
                                            let l = if paired(&rna, c11, c9) {
                                                c.read()
                                                    .unwrap()
                                                    .get(
                                                        n - (c1 as usize) + (c3 as usize) - 2,
                                                        c11 - 1,
                                                    )
                                                    .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(c11 + 1, c9 - 1)
                                                        .unwrap()
                                            } else {
                                                0
                                            };
                                            c.write()
                                                .unwrap()
                                                .set(
                                                    n - c1 + (c3 as usize) - 2usize,
                                                    c9,
                                                    c.read()
                                                        .unwrap()
                                                        .get(n - c1 + (c3 as usize) - 2usize, c9)
                                                        .unwrap()
                                                        + l,
                                                )
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                        } else {
                            for c5 in 0..=8 * c3 {
                                for c9 in n - c1 + 129 * (c3 as usize)
                                    ..=min(n, n - c1 + 129 * (c3 as usize) + 127)
                                {
                                    for c11 in n - c1 + (c3 as usize) + 16usize * (c5 as usize) - 2
                                        ..=min(
                                            min(
                                                n - c1 + 129usize * (c3 as usize),
                                                n - c1
                                                    + (c3 as usize)
                                                    + 16usize * (c5 as usize)
                                                    + 13usize,
                                            ),
                                            c9 - 1,
                                        )
                                    {
                                        let l = if paired(&rna, c11, c9) { 1 } else { 0 };
                                        c.write()
                                            .unwrap()
                                            .set(
                                                n - c1 + (c3 as usize) - 2usize,
                                                c9,
                                                l + c
                                                    .read()
                                                    .unwrap()
                                                    .get(n - c1 + (c3 as usize) - 2usize, c11 - 1)
                                                    .unwrap()
                                                    + c.read()
                                                        .unwrap()
                                                        .get(c11 + 1, c9 - 1)
                                                        .unwrap(),
                                            )
                                            .unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    }
    if kind == 5 {
        for w0 in floord((-(n as i32) - 14i32) as f64, 32f64)..floord(n as f64, 32f64) {
            thread_build.as_ref().unwrap().broadcast(|_| {
                for h0 in max(
                    -(((n as i32) + 13i32) / 16i32),
                    (w0 as i32) - ((n as i32) + 32i32) / 32i32 + 1i32,
                )..=min(-1i32, 2i32 * (w0 as i32) + 2i32)
                {
                    for i0 in max(
                        max(
                            -(n as i32) + 2i32,
                            -32 * (w0 as i32) + 32 * (h0 as i32) - 29,
                        ),
                        16 * h0,
                    )..=16 * h0 + 15
                    {
                        for i1 in max(32 * w0 as i32 - 32 * h0 as i32, -i0 as i32 + 2)
                            ..=min(n as i32, 32 * w0 as i32 - 32 * h0 as i32 + 31)
                        {
                            for i3 in
                                ((c.read().unwrap().row_len() as i32) + (-i0)) as usize..i1 as usize
                            {
                                let l = if paired(&rna, i3 as usize, i1 as usize) {
                                    ck.get(
                                        (c.read().unwrap().row_len() as i32 + (-i0)) as usize,
                                        (i3 - 1) as usize,
                                    )
                                    .unwrap()
                                        + ck.get((i3 + 1) as usize, (i1 - 1) as usize).unwrap()
                                } else {
                                    0
                                };
                                c.write()
                                    .unwrap()
                                    .set(
                                        (c.read().unwrap().row_len() as i32 + (-i0)) as usize,
                                        i1 as usize,
                                        c.write()
                                            .unwrap()
                                            .get(
                                                (c.read().unwrap().row_len() as i32 + (-i0))
                                                    as usize,
                                                i1 as usize,
                                            )
                                            .unwrap()
                                            + l,
                                    )
                                    .unwrap();
                                c.write()
                                    .unwrap()
                                    .set(
                                        (c.read().unwrap().row_len() as i32 + (-i0)) as usize,
                                        i1 as usize,
                                        c.read()
                                            .unwrap()
                                            .get(
                                                (c.read().unwrap().row_len() as i32 + (-i0))
                                                    as usize,
                                                i1 as usize,
                                            )
                                            .unwrap()
                                            + c.read()
                                                .unwrap()
                                                .get(
                                                    (c.read().unwrap().row_len() as i32 + (-i0))
                                                        as usize,
                                                    (i1 - 1) as usize,
                                                )
                                                .unwrap(),
                                    )
                                    .unwrap();
                            }
                        }
                    }
                }
            });
        }
    }
    let end = start.elapsed().as_secs_f64();
    println!("The execution took {} seconds", end);
    if check == 1 {
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
