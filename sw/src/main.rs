use array2d::{Array2D, Error};
use mem::Matrix;
use mem::TSMatrix;
use mem::mem;
use mem::mem_lock;
use mem::rand_seq;
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
mod methods;
fn main() {
    let check_valid = true;
    let mut n = 100;
    let mut dim = 102;
    let mut num_proc = 1;
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
    let mut h = mem_lock(dim);
    let mut h1 = mem_lock(dim);
    let mut m1 = mem_lock(dim);
    let mut m2 = mem_lock(dim);
    let mut w: Vec<i32> = vec![];
    let mut a: Vec<char> = vec![];
    let mut b: Vec<char> = vec![];
    for i in 0..=n {
        h.write().unwrap()[(i, 0)] = 0;
        h.write().unwrap()[(0, i)] = 0;
        h1.write().unwrap()[(i, 0)] = 0;
        h1.write().unwrap()[(0, i)] = 0;
    }
    w[0] = 2;
    for i in 1..=n {
        w.push(i as i32 * w[0]);
    }
    rand_seq(&mut a, n);
    rand_seq(&mut b, n);
    let start = Instant::now();
    if kind == 2 {
        methods::sw_pluto(&mut m1, &mut m2, &mut h, &mut w, &mut a, &mut b, n);
    }
    if kind == 3 {
        methods::sw_traco(&mut m1, &mut m2, &mut h, &mut w, &mut a, &mut b, n);
    }
    if kind == 4 {
        methods::sw_tstile(&mut m1, &mut m2, &mut h, &mut w, &mut a, &mut b, n);
    }
    if kind == 1 || check_valid == true {
        methods::sw_seq(&mut m1, &mut m2, &mut h, &mut w, &mut a, &mut b, n);
    }
}
