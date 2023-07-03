pub mod genchars;

use clap::{Parser, ValueEnum};
use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;
use std::time::{Instant, Duration};

use crate::genchars::gen_all_chars;

#[derive(Parser)]
#[command(author="kekopom & Macronimous", version, about, long_about = "An amazing wordlist generator made in rust.")]
struct Cli {
    /// min size of the password
    #[arg(long)]
    min_size: i8,
    /// max size of the password
    #[arg(long)]
    max_size: i8,
    /// name of the output file
    #[arg(long, short, default_value_t = String::from("output.txt"))]
    output_file_name: String,
    /// Delay in ms to wait between every permutation
    #[arg(long, short, default_value_t = 0)]
    delay_ms: u64,
    /// output mode
    #[arg(short, long, value_enum, default_value_t = OutputMode::File)]
    mode: OutputMode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputMode {
    /// Output will be console
    Console,
    /// Output will be file
    File,
}

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
