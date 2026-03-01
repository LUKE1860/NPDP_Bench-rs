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
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn paired(rna: &Vec<char>, i: usize, j: usize) -> f64 {
    let nt1 = rna[i];
    let nt2 = rna[j];
    match (nt1, nt2) {
        ('A', 'U') => return 1.0,
        ('U', 'A') => return 1.0,
        ('G', 'C') => return 1.0,
        ('C', 'G') => return 1.0,
        ('G', 'U') => return 1.0,
        ('U', 'G') => return 1.0,
        _ => return 0.0,
    }
}
fn main() {
    let check_valid = true;
    let ebp: f64 = 0.0;
    let rt: f64 = 0.0;
    let ert: f64 = f64::exp(-ebp / rt);
    let mut l: i32 = 0;
    let mut delta = 1;
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
    let mut rna: Vec<char> = Vec::new();
    mem::rand_seq(&mut rna, n);
    //memlock
    let mut q = mem::mem_lock(dim);
    let mut q1 = mem::mem(dim);
    //memlock
    let mut qbp = mem::mem_lock(dim);
    let mut qbp1 = mem::mem(dim);
    let mut pbp = mem::mem(dim);
    let mut pu = mem::mem(dim);
    let mut m = mem::mem(dim);
    mem::rna_array_init_lock(&mut q, n, 0.4, 0.4);
    mem::rna_array_init(&mut q1, n, 0.4, 0.4);
    mem::rna_array_init_lock(&mut qbp, n, 0.5, 0.5);
    mem::rna_array_init(&mut qbp1, n, 0.4, 0.4);
    mem::rna_array_init(&mut pbp, n, 0.0, 0.0);
    mem::rna_array_init(&mut pu, n, 0.0, 0.0);
    mem::rna_array_init(&mut q1, n, 0.0, 0.0);
    let start = Instant::now();
    if kind == 1 || check_valid == true {
        // compute the partition functions Q and Qbp
        rayon::scope(|s| {
            s.spawn(|_| {
                if n >= 1 && l >= 0 && l <= 5 {
                    for i in n - 1..=0 {
                        for j in i + 1..n {
                            q1.set(i, j, *q1.get(i, j - 1).unwrap()).unwrap();
                            for k in 0..j - i - (l as usize) {
                                qbp1.set(
                                    k + i,
                                    j,
                                    q1.get(k + i + 1, j - 1).unwrap()
                                        * ert
                                        * paired(&rna, k + i, j - 1),
                                )
                                .unwrap();
                                q1.set(
                                    i,
                                    j,
                                    q1.get(i, j).unwrap()
                                        + (q1.get(i, k + i).unwrap() * qbp1.get(k + i, j).unwrap()),
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            });
        });
    }
    if kind == 2 {
        if (n >= 2) && (l >= 0) && (l <= 5) {
            for t1 in 1..=n - 1 {
                for t2 in 0..=floord((t1 - 1) as f64, 16f64) {
                    for t3 in 0..=t2 {
                        if (t1 >= l as usize + 1) && (t2 == 0) && (t3 == 0) {
                            qbp.write()
                                .unwrap()
                                .set(
                                    0,
                                    t1,
                                    q.read().unwrap().get(1, t1 - 1).unwrap()
                                        * ert
                                        * paired(&rna, 0, t1 - 1),
                                )
                                .unwrap();
                            q.write()
                                .unwrap()
                                .set(0, t1, *q.read().unwrap().get(0, t1 - 1).unwrap())
                                .unwrap();
                            q.write()
                                .unwrap()
                                .set(
                                    0,
                                    t1,
                                    q.read().unwrap().get(0, t1).unwrap()
                                        + (q.read().unwrap().get(0, 0).unwrap()
                                            * qbp.read().unwrap().get(0, t1).unwrap()),
                                )
                                .unwrap();
                        }
                        if t3 == 0 {
                            for t4 in max(1, 16 * t2)..=min(16 * t2 + 15, t1 - l as usize - 1) {
                                qbp.write()
                                    .unwrap()
                                    .set(
                                        t4,
                                        t1,
                                        q.read().unwrap().get(t4 + 1, t1 - 1).unwrap()
                                            * ert
                                            * paired(&rna, 0 + t4, t1 - 1),
                                    )
                                    .unwrap();
                                q.write()
                                    .unwrap()
                                    .set(t4, t1, *q.read().unwrap().get(t4, t1 - 1).unwrap())
                                    .unwrap();
                                q.write()
                                    .unwrap()
                                    .set(
                                        t4,
                                        t1,
                                        q.read().unwrap().get(t4, t4).unwrap()
                                            * qbp.read().unwrap().get(t4, t1).unwrap(),
                                    )
                                    .unwrap();
                                for t5 in 1..=min(15, t4) {
                                    qbp.write()
                                        .unwrap()
                                        .set(
                                            t5 + (t4 - t5),
                                            t1,
                                            q.read()
                                                .unwrap()
                                                .get(t5 + (t4 - t5) + 1, t1 - 1)
                                                .unwrap()
                                                * ert
                                                * paired(&rna, t5 + (t4 - t5), t1 - 1),
                                        )
                                        .unwrap();
                                    q.write()
                                        .unwrap()
                                        .set(
                                            t4 - t5,
                                            t1,
                                            q.read().unwrap().get(t4 - t5, t1).unwrap()
                                                + (q.read()
                                                    .unwrap()
                                                    .get(t4 - t5, t5 + (t4 - t5))
                                                    .unwrap()
                                                    * qbp
                                                        .read()
                                                        .unwrap()
                                                        .get(t5 + (t4 - t5), t1)
                                                        .unwrap()),
                                        )
                                        .unwrap();
                                }
                                if t3 == 0 {
                                    for t4 in max(16 * t2, t1 - 1)..=min(t1 - 1, 16 * t2 + 15) {
                                        q.write()
                                            .unwrap()
                                            .set(
                                                t4,
                                                t1,
                                                *q.read().unwrap().get(t4, t1 - 1).unwrap(),
                                            )
                                            .unwrap();
                                    }
                                }
                                if t3 >= 1 {
                                    for t4 in 16 * t2..=min(16 * t2 + 15, t1 - l as usize - 1) {
                                        for t5 in 16 * t3..=min(t4, 16 * t3 + 15) {
                                            qbp.write()
                                                .unwrap()
                                                .set(
                                                    t5 + (t4 - t5),
                                                    t1,
                                                    q.read()
                                                        .unwrap()
                                                        .get(t5 + (t4 - t5) + 1, t1 - 1)
                                                        .unwrap()
                                                        * ert
                                                        * paired(&rna, t5 + (t4 - t5), t1 - 1),
                                                )
                                                .unwrap();
                                            q.write()
                                                .unwrap()
                                                .set(
                                                    t4 - t5,
                                                    t1,
                                                    q.read().unwrap().get(t4 - t5, t1).unwrap()
                                                        + (q.read()
                                                            .unwrap()
                                                            .get(t4 - t5, t5 + (t4 - t5))
                                                            .unwrap()
                                                            * qbp
                                                                .read()
                                                                .unwrap()
                                                                .get(t5 + (t4 - t5), t1)
                                                                .unwrap()),
                                                )
                                                .unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if kind == 5 {
        if (l >= 0) && (l <= 5) && (n > 5) {
            for w0 in -1..=(n as i32 - 1) / 16 {
                rayon::broadcast(|_| {
                    for h0 in
                        max(w0 - (n as i32 + 15) / 16 + 1, -((n as i32 + 13) / 16))..=min(0, w0)
                    {
                        for t0 in max(0, 8 * w0)
                            ..=min(
                                min(
                                    min(8 * w0 + 15, 8 * w0 - 8 * h0 + 7),
                                    8 * h0 + (n as i32) / 2 + 7,
                                ),
                                (n as i32 + 1) / 2 - 1,
                            )
                        {
                            for i0 in max(
                                max(
                                    max(max(-(n as i32) + 2, -16 * w0 + 16 * h0 - 14), 16 * h0),
                                    -16 * w0 + 16 * h0 + 2 * t0 - 15,
                                ),
                                -(n as i32) + 2 * t0 + 1,
                            )
                                ..min(min(9, 16 * h0 + 15), -16 * w0 + 16 * h0 + 2 * t0 + 1)
                            {
                                for i1 in max(max(16 * w0 - 16 * h0, 2 * t0 - i0), -i0 + 1)
                                    ..=min(
                                        min(n as i32 - 1, 16 * w0 - 16 * h0 + 15),
                                        2 * t0 - i0 + 1,
                                    )
                                {
                                    q.write()
                                        .unwrap()
                                        .set(
                                            q.read().unwrap().row_len() - (i0 as usize),
                                            i1 as usize,
                                            *q.read()
                                                .unwrap()
                                                .get(
                                                    q.read().unwrap().row_len() - (i0 as usize),
                                                    (i1 - 1) as usize,
                                                )
                                                .unwrap(),
                                        )
                                        .unwrap();
                                    for i3 in 0..-l + i0 + i1 {
                                        qbp.write()
                                            .unwrap()
                                            .set(
                                                (-i0 + i3) as usize,
                                                i1 as usize,
                                                (q.read()
                                                    .unwrap()
                                                    .get((-i0 + i3 + 1) as usize, (i1 - 1) as usize)
                                                    .unwrap()
                                                    * ert)
                                                    * paired(
                                                        &rna,
                                                        (-i0 + i3) as usize,
                                                        (i1 - 1) as usize,
                                                    ),
                                            )
                                            .unwrap();
                                        q.write()
                                            .unwrap()
                                            .set(
                                                q.read().unwrap().row_len() - (i0 as usize),
                                                i1 as usize,
                                                q.read()
                                                    .unwrap()
                                                    .get(
                                                        q.read().unwrap().row_len() - (i0 as usize),
                                                        i1 as usize,
                                                    )
                                                    .unwrap()
                                                    + (q.read()
                                                        .unwrap()
                                                        .get(
                                                            q.read().unwrap().row_len()
                                                                - (i0 as usize),
                                                            (-i0 + i3) as usize,
                                                        )
                                                        .unwrap()
                                                        * qbp
                                                            .read()
                                                            .unwrap()
                                                            .get((-i0 + i3) as usize, i1 as usize)
                                                            .unwrap()),
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
    }
    if kind == 3 {
        if n >= 10 && l >= 0 && l <= 5 {
            for c1 in 1..n + (n - 2) / 16 {
                rayon::broadcast(|_| {
                    for c3 in max(0, -(n as i32) + c1 as i32 + 1)..=(c1 - 1) as i32 / 17 {
                        for c4 in 0..=1 {
                            if c4 == 1 {
                                for c5 in 0..=c3 {
                                    for c9 in (n as i32) - (c1 as i32) + 17 * c3
                                        ..=min(
                                            (n as i32) - 1,
                                            (n as i32) - (c1 as i32) + 17 * c3 + 15,
                                        )
                                    {
                                        if c5 == c3
                                            && c1 + (c9 as usize) >= n + 17 * (c3 as usize) + 1
                                        {
                                            q.write()
                                                .unwrap()
                                                .set(
                                                    n - (c1 as usize) + (c3 as usize) - 1,
                                                    c9 as usize,
                                                    *q.read()
                                                        .unwrap()
                                                        .get(
                                                            n - c1 + (c3 as usize) - 1,
                                                            (c9 - 1) as usize,
                                                        )
                                                        .unwrap(),
                                                )
                                                .unwrap();
                                        }
                                        if c5 == c3
                                            && c1 + (c9 as usize) >= n + 17 * (c3 as usize) + 1
                                        {
                                            for c11 in 0..16 * c3 {
                                                q.write()
                                                    .unwrap()
                                                    .set(
                                                        n - c1 + c3 as usize - 1,
                                                        c9 as usize,
                                                        q.read()
                                                            .unwrap()
                                                            .get(
                                                                n - c1 + c3 as usize - 1,
                                                                c9 as usize,
                                                            )
                                                            .unwrap()
                                                            + (q.read()
                                                                .unwrap()
                                                                .get(
                                                                    n - c1 + c3 as usize - 1,
                                                                    c11 as usize
                                                                        + (n - c1 + c3 as usize
                                                                            - 1),
                                                                )
                                                                .unwrap()
                                                                * qbp
                                                                    .read()
                                                                    .unwrap()
                                                                    .get(
                                                                        c11 as usize
                                                                            + (n - c1
                                                                                + c3 as usize
                                                                                - 1),
                                                                        c9 as usize,
                                                                    )
                                                                    .unwrap()),
                                                    )
                                                    .unwrap();
                                            }
                                        }
                                        for c11 in 16 * c5
                                            ..=min(16 * c5 + 15, -(n as i32) + c1 as i32 - c3 + c9)
                                        {
                                            qbp.write()
                                                .unwrap()
                                                .set(
                                                    c11 as usize + (n - c1 + c3 as usize - 1),
                                                    c9 as usize,
                                                    q.read()
                                                        .unwrap()
                                                        .get(
                                                            c11 as usize
                                                                + (n - c1 + c3 as usize - 1)
                                                                + 1,
                                                            (c9 - 1) as usize,
                                                        )
                                                        .unwrap()
                                                        * ert
                                                        * paired(
                                                            &rna,
                                                            c11 as usize
                                                                + (n - c1 + c3 as usize - 1),
                                                            (c9 - 1) as usize,
                                                        ),
                                                )
                                                .unwrap();
                                            if c5 == c3 {
                                                q.write()
                                                    .unwrap()
                                                    .set(
                                                        n - c1 + c3 as usize - 1,
                                                        c9 as usize,
                                                        q.read()
                                                            .unwrap()
                                                            .get(
                                                                n - c1 + c3 as usize - 1,
                                                                c9 as usize,
                                                            )
                                                            .unwrap()
                                                            + (q.read()
                                                                .unwrap()
                                                                .get(
                                                                    n - c1 + c3 as usize - 1,
                                                                    c11 as usize
                                                                        + (n - c1 + c3 as usize
                                                                            - 1),
                                                                )
                                                                .unwrap()
                                                                * qbp
                                                                    .read()
                                                                    .unwrap()
                                                                    .get(
                                                                        c11 as usize + n - c1
                                                                            + c3 as usize
                                                                            - 1,
                                                                        c9 as usize,
                                                                    )
                                                                    .unwrap()),
                                                    )
                                                    .unwrap();
                                            } else if c1 + c9 as usize == n + 17 * c3 as usize {
                                                q.write()
                                                    .unwrap()
                                                    .set(
                                                        n - c1 + c3 as usize - 1,
                                                        n - c1 + 17 * c3 as usize,
                                                        q.read()
                                                            .unwrap()
                                                            .get(
                                                                n - c1 + c3 as usize - 1,
                                                                n - c1 + 17 * c3 as usize,
                                                            )
                                                            .unwrap()
                                                            + (q.read()
                                                                .unwrap()
                                                                .get(
                                                                    n - c1 + c3 as usize - 1,
                                                                    c11 as usize
                                                                        + (n - c1 + c3 as usize
                                                                            - 1),
                                                                )
                                                                .unwrap()
                                                                * qbp
                                                                    .read()
                                                                    .unwrap()
                                                                    .get(
                                                                        c11 as usize
                                                                            + (n - c1
                                                                                + c3 as usize
                                                                                - 1),
                                                                        n - c1 + 17 * c3 as usize,
                                                                    )
                                                                    .unwrap()),
                                                    )
                                                    .unwrap();
                                            }
                                        }
                                    }
                                }
                            } else {
                                q.write()
                                    .unwrap()
                                    .set(
                                        n - c1 + c3 as usize - 1,
                                        n - c1 + 17 * c3 as usize,
                                        *q.read()
                                            .unwrap()
                                            .get(
                                                n - c1 + c3 as usize - 1,
                                                (n - c1 + 17 * c3 as usize) - 1,
                                            )
                                            .unwrap(),
                                    )
                                    .unwrap()
                            }
                        }
                    }
                });
            }
        }
    }
    let stop = start.elapsed().as_secs_f64();
    println!("It took {stop} seconds");
    if check_valid == true {
        for i in 0..n {
            for j in 0..n {
                if q.read().unwrap().get(i, j).unwrap() != q1.get(i, j).unwrap() {
                    println!(
                        "Error {} {} {} {} ",
                        q.read().unwrap().get(i, j).unwrap(),
                        q1.get(i, j).unwrap(),
                        i,
                        j
                    )
                }
            }
        }
    }
}
