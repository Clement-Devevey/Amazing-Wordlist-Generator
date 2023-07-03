use clap::Parser;

use crate::output_mode::OutputMode;

#[derive(Parser)]
#[command(author="kekopom & Macronimous", version, about, long_about = "An amazing wordlist generator made in rust.")]
pub struct Cli {
    /// min size of the password
    #[arg(long)]
    pub min_size: i8,
    /// max size of the password
    #[arg(long)]
    pub max_size: i8,
    /// name of the output file
    #[arg(long, short, default_value_t = String::from("output.txt"))]
    pub output_file_name: String,
    /// Delay in ms to wait between every permutation
    #[arg(long, short, default_value_t = 0)]
    pub delay_ms: u64,
    /// output mode
    #[arg(short, long, value_enum, default_value_t = OutputMode::File)]
    pub mode: OutputMode,
}