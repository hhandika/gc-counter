//! Heru Handika
//! 23 December 2020
//! Read fasta file and count gc content
//! Support multiline.
mod counter;

use counter::fasta as fast;

use std::string::String;

fn main() {
    let f = String::from("data/b_chrysocomus.fasta");
    let seq = fast::read_file(&f);
    fast::write_results(&seq);
}

