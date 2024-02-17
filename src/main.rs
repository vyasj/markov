use std::collections::HashMap;
use std::env;
use std::fs;

struct MarkovNode {
    freq: u64,
    next: HashMap<String, MarkovNode>
}

fn main() {
    let filename = env::args().nth(1).expect("Expected a single command line argument");
    let mut markov_chain: HashMap<String, MarkovNode> = HashMap::new();

    let binding = fs::read_to_string(&filename)
        .expect(&format!("Unable to read {}", &filename));
    let instructions: Vec<&str> = binding
        .lines()
        .collect();

    for line in instructions {
        let words_vec: Vec<&str> = line.split(" ").collect();
        let command = words_vec[0];
        let infile = words_vec[1];
        let windowsize: usize = words_vec[2].parse().unwrap();
        
        match command {
            "load" => {
                let binding = fs::read_to_string(&infile)
                    .expect(&format!("Unable to read {}", &infile));
                let words: Vec<&str> = binding
                    .split(" ")
                    .collect();
                
                for (i, _) in words.iter().enumerate() {
                    populate(&words, i, windowsize, &mut markov_chain);
                }
            },
            "generate" => generate(),
            "quit" => break,
            _ => { 
                println!("Unrecognized command: {}", command);
                std::process::exit(1);
            },
        }
    }
}

fn populate(text: &Vec<&str>, index: usize, context_depth: usize, data: &mut HashMap<String, MarkovNode>) {
    println!("Populating...");

    if context_depth == 0 || index >= text.len() {
        return;
    }

    if let Some(exists) = data.get_mut(text[index]) {
        exists.freq = exists.freq + 1;
        populate(text, index+1, context_depth-1, &mut exists.next);
    } else {
        // Create node and add to map
    }
}

fn generate() {
    println!("Generating...");
}