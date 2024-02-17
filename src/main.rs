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
    let mut max_windowsize: usize = 0;

    let binding = fs::read_to_string(&filename)
        .expect(&format!("Unable to read {}", &filename));
    let instructions: Vec<&str> = binding
        .lines()
        .collect();

    for line in instructions {
        let words_vec: Vec<&str> = line.split(" ").collect();
        let command = words_vec[0];
        
        match command {
            "load" => {
                let infile = words_vec[1];
                let windowsize: usize = words_vec[2].parse().unwrap();
                max_windowsize = std::cmp::max(windowsize, max_windowsize);

                let binding = fs::read_to_string(&infile)
                    .expect(&format!("Unable to read {}", &infile));
                let words: Vec<&str> = binding
                    .split(" ")
                    .collect();
                
                for (i, _) in words.iter().enumerate() {
                    populate(&words, i, windowsize, &mut markov_chain);
                }
            },
            "generate" => {
                let gen_mode = words_vec[1];
                let start_word = words_vec[2];
                let windowsize: usize = words_vec[3].parse().unwrap();

                if windowsize > max_windowsize {
                    println!("Invalid context size: {}. Try with a context size equal to or smaller than the largest size provided in the \"load\" commands.", windowsize);
                }

                let output: String = generate(gen_mode, start_word, windowsize, &markov_chain);

                println!("{}", output);
            },
            "quit" => break,
            _ => { 
                println!("Unrecognized command: {}", command);
            },
        }
    }
}

fn populate(text: &Vec<&str>, index: usize, context_depth: usize, data: &mut HashMap<String, MarkovNode>) {
    if context_depth == 0 || index >= text.len() {
        return;
    }

    let mut curr_word = text[index].to_string();
    curr_word.retain(|c| !r#"(),".;:'"#.contains(c));

    if let Some(next_node) = data.get_mut(&curr_word) {
        next_node.freq = next_node.freq + 1;
        populate(text, index+1, context_depth-1, &mut next_node.next);
    } else {
        let tmp_node = MarkovNode {
            freq: 1,
            next: HashMap::new(),
        };
        data.insert(curr_word.clone(), tmp_node);
        if let Some(next_node) = data.get_mut(&curr_word) {
            populate(text, index+1, context_depth-1, &mut next_node.next);
        } else {
            println!("Something went wrong.");
            std::process::exit(1);
        }
    }
}

fn generate(mode: &str, first_word: &str, context_depth: usize, data: &HashMap<String, MarkovNode>) -> String {
    if data.get(first_word).is_none() {
        println!("The word: {} does not exist in the text. Try something else.", first_word);
        std::process::exit(1);
    }
    
    let mut gen_string = String::from(first_word);
    let mut counter = context_depth;
    let mut key_val = String::from(first_word);

    match mode {
        "most_common" => {
            loop {
                if counter == 0 {
                    break;
                }
                let most_common_next: String = find_most_common(&data[&key_val].next);
                gen_string.push_str(" ");
                gen_string.push_str(&most_common_next);
                key_val = most_common_next;
                counter = counter - 1;
            }
        },
        "random" => {
            println!("Not built yet. Use most_common for now.");
        },
        _ => { 
            println!("Unrecognized mode: {}", mode);
            std::process::exit(1);
        },
    }

    gen_string
}

fn find_most_common(data: &HashMap<String, MarkovNode>) -> String {
    let mut highest_freq: u64 = 0;
    let mut return_key: String = String::new();
    for key in data.keys() {
        if data[key].freq > highest_freq {
            highest_freq = data[key].freq;
            return_key = key.to_string();
        }
    }

    return_key
}