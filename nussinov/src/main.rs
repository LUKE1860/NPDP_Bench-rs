#![feature(f128)]
use array2d::{Array2D, Error};
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
mod library;
mod methods;
fn main() {
    let mut num_proc: usize = 1usize;
    let mut n: usize = 10;
    let args: Vec<String> = env::args().collect();
    if args.len() > 4 {
        println!("./nussinov [method:oryg,tstile,tilecorr,pluto] [number of threads] [n]")
    }
    let mut method: String = "oryg".to_string();
    if args.len() == 4 {
        match args[1].parse::<String>() {
            Ok(value) => method = value,
            Err(e) => eprintln!("{}", e),
        }
        match args[2].parse::<usize>() {
            Ok(value) => num_proc = value,
            Err(e) => eprintln!("{}", e),
        }
        match args[3].parse::<usize>() {
            Ok(value) => n = value,
            Err(e) => eprintln!("{}", e),
        }
    }
    let s = library::mem(n);
    let mut rna: Vec<char> = Vec::with_capacity(n + 5);
    println!("Method {method}");
    println!("N {n}");
    let start = Instant::now();
    match method.as_str() {
        "oryg" => methods::oryg(s, n, rna),
        "tstile" => methods::tstile(s, n, rna),
        "tilecorr" => methods::tilecorr(s, n, rna),
        "pluto" => methods::pluto(s, n, rna),
        _ => println!("Wrong method"),
    }
    let stop = start.elapsed().as_secs_f64();
    println!("It took {stop} seconds");
}
