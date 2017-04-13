extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=words.txt");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "static WORDS: phf::OrderedSet<&'static str> = ").unwrap();
    let mut builder = phf_codegen::OrderedSet::new();

    let word_file = File::open("words.txt").unwrap();
    let reader = BufReader::new(word_file);
    for line in reader.lines() {
        builder.entry(line.unwrap());
    }

    builder.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}
