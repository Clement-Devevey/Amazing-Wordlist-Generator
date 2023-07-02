
pub mod genchars;

use clap::Parser;
use std::mem;
use itertools::{Itertools};
use std::time::{Instant};

use crate::genchars::gen_all_chars;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// min size of the password
    #[arg(long)]
    min_size: i8,
    /// max size of the password
    #[arg(long)]
    max_size: i8,
}

fn main() {
    let start = Instant::now();
    let mut args = Cli::parse();

    if args.min_size > args.max_size {
        mem::swap(&mut args.min_size, &mut args.max_size);
    }
    println!("Password lenght: {:?}, {:?} ", args.min_size, args.max_size );
    
    let characters = gen_all_chars();

    let mut all_perm : Vec<Vec<char>> = Vec::new();
    for n in args.min_size..args.max_size+1 {
        _ = characters.clone().into_iter().permutations(n.try_into().unwrap()).for_each(|f| {all_perm.push(f);});
    }
    let duration = start.elapsed();

    let start_print = Instant::now();
    for perm in &all_perm {
       println!("{:?}", perm);
    }
    let duration_print = start_print.elapsed();
    println!("generated {} permutation, with {} possible characters in {:?}. time to display all these is {:?}", all_perm.len(), characters.len(), duration, duration_print);
}