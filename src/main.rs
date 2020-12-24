//! Heru Handika
//! 23 December 2020
//! Read fasta file and count gc content
//! Support multiline.
mod counter;

use clap::{App, Arg};
use counter::fasta;

use std::string::String;

fn main() {
    let args = App::new("GC-Counter")
        .version("0.0.1")
        .about("Quickly count gc content from a fasta file.")
        .arg(Arg::with_name("input")
            .help("Fasta file to analyze.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .help("Output filename")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input = args.value_of("input").unwrap();
    let output = args.value_of("output").unwrap();
    let infile = String::from(input);
    let outfile = String::from(output);

    fasta::parse_fasta(&infile, &outfile);
}

