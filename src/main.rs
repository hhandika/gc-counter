//! Heru Handika
//! 23 December 2020
//! Read fasta file and count gc content
//! Support multiline.
mod counter;

use clap::{App, Arg};
use counter::fasta as fast;

use std::string::String;

fn main() {
    let args = App::new("GC-Counter")
        .version("0.0.1")
        .about("Quickly count gc content from a fasta file.")
        .arg(Arg::with_name("input")
            .help("Fasta file to analyze.")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input = args.value_of("input").unwrap();
    let f = String::from(input);
    let seq = fast::read_file(&f);
    fast::print_results(&seq);
}

