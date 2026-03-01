//FIX TRIANG
// Check the source again
//
// FIX LOCKING IN OTHER KINDS
use array2d::{Array2D, Error};
use mem::Matrix;
use parking_lot::deadlock;
use rand::prelude::*;
use rayon::ThreadPool;
use rayon::ThreadPoolBuildError;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::env;
use std::sync::Barrier;
use std::sync::Condvar;
use std::thread;
use std::time::{Duration, Instant};
mod mem;
fn min_f64(f_num: f64, s_num: f64) -> f64 {
    if f_num == i32::MAX as f64 && s_num == 0.0 {
        return 0.0;
    }
    return 0.0;
}
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn ceild(f_num: f64, s_num: f64) -> usize {
    return f64::ceil(f_num / s_num) as usize;
}
fn dist(p1: &Vec<i32>, p2: &Vec<i32>) -> f64 {
    return f64::sqrt(
        ((p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1])) as f64,
    );
}
fn cost(points: &Vec<Vec<i32>>, i: usize, j: usize, k: usize) -> f64 {
    let p1 = &points[i];
    let p2 = &points[j];
    let p3 = &points[k];
    return dist(p1, p2) + dist(p2, p3) + dist(p3, p1);
}
fn mctdp(matrix: Matrix<i32>, kind: i32, n: usize, dim: usize) -> f64 {
    let mut table = mem::mem_lock(dim);
    let start = Instant::now();
    if kind == -1 {}
    //it goes for far too long
    if kind == 1 {
        for gap in 0..n {
            for j in gap..n {
                if gap < 2 {
                    table.get_mut()[(j - gap, j)] = 0.0;
                } else {
                    table.get_mut()[(j - gap, j)] = i32::MAX as f64;
                    for k in j - gap + 1..j {
                        table.get_mut()[(j - gap, j)] = f64::min(
                            table.get_mut()[(j - gap, j)],
                            table.get_mut()[(j - gap, k)]
                                + table.get_mut()[(k, j)]
                                + cost(&matrix.as_rows(), j - gap, j, k),
                        );
                    }
                }
            }
        }
    }
    if kind == 2 {
        if n >= 1 {
            for t1 in 0..=floord((n - 1) as f64, 8f64) {
                let lbp = ceild(t1 as f64, 2f64);
                let ubp = min(floord((n - 1) as f64, 16f64), t1);
                rayon::broadcast(|_| {
                    for t2 in lbp..=ubp {
                        if t1 == t2 {
                            for t3 in 0..min(1, n - 1) {
                                for t4 in max(16 * t1, t3)..=min(n - 1, 16 * t1 + 15) {
                                    table.write()[(t4 - t3, t4)] = 0.0;
                                }
                            }
                        }
                        for t3 in max(2, 16 * t1 - 16 * t2)..=min(n - 1, 16 * t1 - 16 * t2 + 15) {
                            for t4 in max(16 * t2, t3)..=min(n - 1, 16 * t2 + 15) {
                                table.write()[(t4 - t3, t4)] = i32::MAX as f64;
                                for t5 in -(t3 as i32) + t4 as i32 + 1..=t4 as i32 - 1 {
                                    table.write()[(t4 - t3, t4)] = f64::min(
                                        table.read()[(t4 - t3, t4)],
                                        table.read()[(t4 - t3, t5 as usize)]
                                            + table.read()[(t5 as usize, t4)]
                                            + cost(&matrix.as_rows(), t4 - t3, t4, t5 as usize),
                                    );
                                }
                                /*
                                table.write().unwrap()[(t4 - t3, t4)] = i32::MAX as f64;
                                for t5 in -(t3 as i32) + t4 as i32 + 1..=t4 as i32 - 1 {
                                    table.write().unwrap()[(t4 - t3, t4)] = f64::min(
                                        table.read().unwrap()[(t4 - t3, t4)],
                                        table.read().unwrap()[(t4 - t3, t5 as usize)]
                                            + table.read().unwrap()[(t5 as usize, t4)]
                                            + cost(&matrix.as_rows(), t4 - t3, t4, t5 as usize),
                                    );
                                }
                                */
                            }
                        }
                    }
                });
            }
        }
    }
    /*
    if kind == 3 {
        if n >= 1 {
            let lbp = 0;
            let ubp = floord((n - 1) as f64, 16f64);
            rayon::broadcast(|_| {
                let mut lbv: usize;
                let mut ubv: usize;
                for t2 in lbp..=ubp {
                    for t3 in t2..=floord((n - 1) as f64, 16f64) {
                        if t2 == 0 {
                            for t4 in 0..min(1, n - 1) {
                                lbv = max(16 * t3, t4);
                                ubv = min(n - 1, 16 * t3 + 15);
                                for t5 in lbv..ubv {
                                    table.write().unwrap()[(t5 - t4, t5)] = 0.0;
                                }
                            }
                        }
                        for t4 in max(2, 16 * t2)..=min(n - 1, 16 * t2 + 15) {
                            lbv = max(16 * t3, t4);
                            ubv = min(n - 1, 16 * t3 + 15);
                            for t5 in lbv..=ubv {
                                table.write().unwrap()[(t5 - t4, t5)] = i32::MAX as f64;
                            }
                        }
                    }
                }
                for c1 in 4..2 * n - 1 {
                    rayon::broadcast(|_| {
                        for c3 in -((c1 as i32 - 1) % 2) + 1
                            ..=min(c1 as i32 - 4, (2 * n as i32 - c1 as i32 - 2) / 31)
                        {
                            for c5 in 0..=(c1 - c3 as usize - 4) / 32 {
                                for c9 in (c1 + 31 * c3 as usize) / 2
                                    ..=min(n - 1, ((c1 + 31 * c3 as usize) / 2) + 15)
                                {
                                    for c11 in ((-(c1 as i32) + c3 as i32) / 2)
                                        + 16 * c5 as i32
                                        + c9 as i32
                                        + 1
                                        ..=min(
                                            c9 as i32 - 1,
                                            ((-(c1 as i32) + c3 as i32) / 2)
                                                + 16 * c5 as i32
                                                + c9 as i32
                                                + 16,
                                        )
                                    {
                                        table.write().unwrap()
                                            [(c9 - ((c1 - c3 as usize) / 2), c9)] = f64::min(
                                            table.read().unwrap()
                                                [(c9 - ((c1 - c3 as usize) / 2), c9)],
                                            table.read().unwrap()
                                                [(c9 - ((c1 - c3 as usize) / 2), c11 as usize)]
                                                + table.read().unwrap()[(c11 as usize, c9)]
                                                + cost(
                                                    &matrix.as_rows(),
                                                    c9 - ((c1 - c3 as usize) / 2),
                                                    c9,
                                                    c11 as usize,
                                                ),
                                        )
                                    }
                                }
                            }
                        }
                    });
                }
            });
        }
    }
    if kind == 4 {
        if n >= 1 {
            let lbp = 0;
            let ubp = floord((n - 1) as f64, 16f64);
            rayon::broadcast(|_| {
                let mut lbv: usize;
                let mut ubv: usize;
                for t2 in lbp..=ubp {
                    for t3 in t2..=floord((n - 1) as f64, 16f64) {
                        if t2 == 0 {
                            for t4 in 0..min(1, n - 1) {
                                lbv = max(16 * t3, t4);
                                ubv = min(n - 1, 16 * t3 + 15);
                                for t5 in lbv..ubv {
                                    table.write().unwrap()[(t5 - t4, t5)] = 0.0;
                                }
                            }
                        }
                        for t4 in max(2, 16 * t2)..=min(n - 1, 16 * t2 + 15) {
                            lbv = max(16 * t3, t4);
                            ubv = min(n - 1, 16 * t3 + 15);
                            for t5 in lbv..=ubv {
                                table.write().unwrap()[(t5 - t4, t5)] = i32::MAX as f64;
                            }
                        }
                    }
                }
            });
            for c0 in 0..floord((n - 1) as f64, 16f64) {
                rayon::broadcast(|_| {
                    for c1 in max(0, c0 - (n + 13) / 16 + 1)..=c0 {
                        for c3 in max(16 * c0 + 16 * c1, 16 * c0 - 16 * c1 + 4)
                            ..=min(
                                min(2 * n - 16 * c0 + 16 * c1 - 2, n + 16 * c1 + 14),
                                16 * c0 + 16 * c1 + 45,
                            )
                        {
                            for c4 in max(
                                c0 as i32 - c1 as i32,
                                -2 * c1 as i32 + (c3 as i32 + 3) / 16 - 2,
                            )
                                ..=min(
                                    min((n as i32 - 2) / 16, -(c1 as i32) + (c3 as i32 - 1) / 16),
                                    16 * c0 as i32 + 16 * c1 as i32 + c3 as i32 + 13,
                                )
                            {
                                for c6 in max(
                                    max(
                                        max(max(2, 16 * c1 as i32), -(n as i32) + c3 as i32 + 1),
                                        -8 * c0 as i32 + 8 * c1 as i32 + c3 as i32 / 2 - 7,
                                    ),
                                    -8 * c4 as i32 + (c3 as i32 + 1) / 2 - 7,
                                )
                                    ..=min(
                                        min(16 * c1 as i32 + 15, c3 as i32 - 16 * c4 as i32 - 1),
                                        -8 * c0 as i32 + 8 * c1 as i32 + c3 as i32 / 2,
                                    )
                                {
                                    for c10 in max(16 * c4 as i32, c3 as i32 - 2 * c6 as i32 + 1)
                                        ..=min(16 * c4 as i32 + 15, c3 as i32 - c6 as i32 - 1)
                                    {
                                        table.write().unwrap()
                                            [((c3 - c6 as usize) - c6 as usize, c3 - c6 as usize)] =
                                            f64::min(
                                                table.read().unwrap()[(
                                                    (c3 - c6 as usize) - c6 as usize,
                                                    c3 - c6 as usize,
                                                )],
                                                table.read().unwrap()[(
                                                    (c3 - c6 as usize) - c6 as usize,
                                                    c10 as usize,
                                                )] + table.read().unwrap()
                                                    [(c10 as usize, (c3 - c6 as usize))]
                                                    + cost(
                                                        &matrix.as_rows(),
                                                        (c3 - c6 as usize) - c6 as usize,
                                                        c3 - c6 as usize,
                                                        c10 as usize,
                                                    ),
                                            )
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
    }
    */
    let stop = start.elapsed().as_secs_f64();
    println!("It took {stop} seconds");
    return table.read()[(0, n - 1)];
}
fn main() {
    let mut num_proc = 4;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(value) => num_proc = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    let mut kind = 2;
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
    let points = mem::mem_int(dim);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        }
    });
    mctdp(points, kind, n, dim);
}
