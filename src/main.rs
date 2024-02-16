use std::collections::HashMap;
use std::env;
use std::fs;

struct Node {
    freq: u64,
    next: HashMap<String, Node>
}

fn main() {
    let filename = env::args().nth(1).expect("Expected a single command line argument");
    let mut shitkov_chain: HashMap<String, Node> = HashMap::new();

    let instructions: Vec<String> = fs::read_to_string(filename)
        .expect("Unable to read filename")
        .lines()
        .map(String::from)
        .collect();

    for line in instructions {
        println!("{line}");
    }
}

fn populate(text: Vec<&str>, depth: u32, chain: &mut HashMap<String, Node>) {
    for word in text {
        println!("{}", word);
    }
}