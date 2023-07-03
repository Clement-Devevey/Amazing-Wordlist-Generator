pub mod genchars;
pub mod cli;
pub mod output_mode;

use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;
use std::time::{Instant, Duration};

use crate::cli::Cli;
use crate::genchars::gen_all_chars;
use crate::output_mode::OutputMode;

fn main() {
    let start = Instant::now();
    let mut args = Cli::parse();

    if args.min_size > args.max_size {
        mem::swap(&mut args.min_size, &mut args.max_size);
    }
    println!("Password lenght: {:?}, {:?} ", args.min_size, args.max_size);

    let characters = gen_all_chars();
    let mut nb_perm = 0;

    // Choose between console mode or file mode according to user intput
    let mut out_writer = match args.mode {
        OutputMode::Console => Box::new(io::stdout()) as Box<dyn Write>,
        OutputMode::File => {
            let f_password = File::create(args.output_file_name).expect("Unable to create file");
            Box::new(f_password) as Box<dyn Write>
        }
    };
    
    for n in args.min_size..args.max_size + 1 {
        _ = characters
            .clone()
            .into_iter()
            .permutations(n.try_into().unwrap())
            .for_each(|perm| {
                nb_perm += 1;
                _ = write!(out_writer, "{}\n", perm.into_iter().collect::<String>());
                std::thread::sleep(Duration::from_millis(args.delay_ms));
            })
        }
    let duration = start.elapsed();

    println!(
        "generated {} permutation, with {} possible characters in {:?}.",
        nb_perm,
        characters.len(),
        duration
    );
}
