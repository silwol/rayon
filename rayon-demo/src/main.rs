#![cfg_attr(test, feature(test))]

use std::env;
use std::io;
use std::io::prelude::*;
use std::process::exit;

mod cpu_time;
mod life;
mod matmul;
mod mergesort;
mod nbody;
mod noop;
mod quicksort;
mod sieve;
mod tsp;

// these are not "full-fledged" benchmarks yet,
// they only run with cargo bench
#[cfg(test)]
mod factorial;
#[cfg(test)]
mod fibonacci;
#[cfg(test)]
mod find;
#[cfg(test)]
mod join_microbench;
#[cfg(test)]
mod map_collect;
#[cfg(test)]
mod pythagoras;
#[cfg(test)]
mod sort;
#[cfg(test)]
mod str_split;
#[cfg(test)]
mod vec_collect;

#[macro_use]
extern crate serde_derive; // all
#[macro_use]
extern crate glium; // nbody
#[macro_use]
extern crate lazy_static; // find
#[cfg(windows)]
extern crate winapi; // life
#[cfg(test)]
extern crate test;

use rand;
use rand_xorshift;

const USAGE: &str = "
Usage: rayon-demo bench
       rayon-demo <demo-name> [ options ]
       rayon-demo --help

A collection of different benchmarks of Rayon. You can run the full
benchmark suite by executing `cargo bench` or `rayon-demo bench`.

Alternatively, you can run individual benchmarks by running
`rayon-demo foo`, where `foo` is the name of a benchmark. Each
benchmark has its own options and modes, so try `rayon-demo foo
--help`.

Benchmarks:

  - life : Conway's Game of Life.
  - nbody: A physics simulation of multiple bodies attracting and repelling
           one another.
  - sieve: Finding primes using a Sieve of Eratosthenes.
  - matmul: Parallel matrix multiplication.
  - mergesort: Parallel mergesort.
  - noop: Launch empty tasks to measure CPU usage.
  - quicksort: Parallel quicksort.
  - tsp: Traveling salesman problem solver (sample data sets in `data/tsp`).
";

fn usage() -> ! {
    let _ = writeln!(&mut io::stderr(), "{}", USAGE);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    let bench_name = &args[1];
    match &bench_name[..] {
        "matmul" => matmul::main(&args[1..]),
        "mergesort" => mergesort::main(&args[1..]),
        "nbody" => nbody::main(&args[1..]),
        "quicksort" => quicksort::main(&args[1..]),
        "sieve" => sieve::main(&args[1..]),
        "tsp" => tsp::main(&args[1..]),
        "life" => life::main(&args[1..]),
        "noop" => noop::main(&args[1..]),
        _ => usage(),
    }
}

fn seeded_rng() -> rand_xorshift::XorShiftRng {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}
