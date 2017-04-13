extern crate phf;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    println!("{:?}", WORDS.len());
}
