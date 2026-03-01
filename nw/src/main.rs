use array2d::{Array2D, Error};
use mem::Matrix;
use mem::mem;
use mem::mem_lock;
use mem::rand_seq;
use rayon::ThreadPool;
use rayon::ThreadPoolBuildError;
use rayon::ThreadPoolBuilder;
use rayon::current_num_threads;
use std::cmp::{max, min};
use std::env;
use std::time::{Duration, Instant};
mod mem;
fn floord(f_num: f64, s_num: f64) -> usize {
    return f64::floor(f_num / s_num) as usize;
}
fn ceild(f_num: f64, s_num: f64) -> usize {
    return f64::ceil(f_num / s_num) as usize;
}
fn sw_seq(
    m1: &mut Matrix<i32>,
    m2: &mut Matrix<i32>,
    h: &mut Matrix<i32>,
    w: Vec<i32>,
    a: Vec<char>,
    b: Vec<char>,
    n: usize,
) {
    rayon::scope(|x| {
        println!("{}", current_num_threads());
        x.spawn(|_| {
            for i in 1..=n {
                for j in 1..=n {
                    m1[(i, j)] = i32::MIN;
                    for k in 1..=i {
                        m1[(i, j)] = max(m1[(i, j)], h[(i - k, j)] + w[k]);
                    }
                    m2[(i, j)] = i32::MIN;
                    for k in 1..=j {
                        m2[(i, j)] = max(m2[(i, j)], h[(i, j - k)] + w[k]);
                    }
                    h[(i, j)] = max(
                        h[(i - 1, j - 1)] + mem::s(a[i], b[i]),
                        max(m1[(i, j)], m2[(i, j)]),
                    );
                }
            }
        });
    });
}
fn main() {
    let check_valid = true;
    let mut kind = 1;
    let mut n: usize = 100;
    let mut dim: usize = 102;
    let mut num_proc = 8;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(value) => num_proc = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    if args.len() > 2 {
        match args[2].parse::<usize>() {
            Ok(value) => n = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    dim = 2 * n + 2;
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
    let mut f = mem_lock(dim);
    let mut h = mem(dim);
    let mut tmp_f = mem_lock(dim);
    let mut f1 = mem_lock(dim);
    let mut m1 = mem(dim);
    let mut m2 = mem(dim);
    let mut w: Vec<i32> = Vec::with_capacity(dim);
    let mut a: Vec<char> = Vec::with_capacity(dim);
    let mut b: Vec<char> = Vec::with_capacity(dim);
    for i in 0..=n {
        f.write().unwrap()[(i, 0)] = 0;
        f.write().unwrap()[(0, i)] = 0;
        f1.write().unwrap()[(i, 0)] = 0;
        f1.write().unwrap()[(0, i)] = 0;
    }
    w.push(2);
    for i in 1..=n {
        w.push(i as i32 * w[0]);
    }
    rand_seq(&mut a, n);
    rand_seq(&mut b, n);
    let start = Instant::now();
    if kind == 1 || check_valid == true {
        // is swap needed here
        sw_seq(&mut m1, &mut m2, &mut h, w, a, b, n);
        if check_valid == true {
            for i in 0..n {
                for j in 0..n {
                    if f.read().unwrap()[(i, j)] != f1.read().unwrap()[(i, j)] {
                        eprintln!("Error");
                    }
                }
            }
        }
    }
}
