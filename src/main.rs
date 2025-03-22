use std::collections::HashMap;
use std::env;
use std::fs;

// Data Structure for a single Markov node
struct MarkovNode {
    freq: u16,
    next: HashMap<String, MarkovNode>,
}

fn main() {
    // Read the filename passed in as the first command line argument, and initialize empty Markov chain
    let filename = env::args()
        .nth(1)
        .expect("Expected a single command line argument");
    let mut markov_chain: HashMap<String, MarkovNode> = HashMap::new();
    let mut max_windowsize: usize = 0;
    let mut text_dir = String::from("texts/");

    // Read file line by line
    let binding = fs::read_to_string(&filename).expect(&format!("Unable to read {}", &filename));
    let instructions: Vec<&str> = binding.lines().collect();

    // Read each line instruction token by token
    // See README.md for expected instruction file structure
    for line in instructions {
        let command_details: Vec<&str> = line.split(" ").collect();
        let command = command_details[0];

        match command {
            "load" => {
                // Load a text file and train the Markov chain
                let infile = command_details[1];
                let windowsize: usize = command_details[2].parse().unwrap();
                max_windowsize = std::cmp::max(windowsize, max_windowsize);

                text_dir.push_str(&infile);
                let binding =
                    fs::read_to_string(&text_dir).expect(&format!("Unable to read {}", &infile));
                let lines: Vec<&str> = binding.lines().collect();

                for line in lines {
                    let words: Vec<&str> = line.split(" ").collect();

                    for (i, _) in words.iter().enumerate() {
                        populate(&words, i, windowsize, &mut markov_chain);
                    }
                }
            }
            "generate" => {
                // Generate a sentence
                let gen_mode = command_details[1];
                let start_word = command_details[2];
                let windowsize: usize = command_details[3].parse().unwrap();

                if windowsize > max_windowsize {
                    println!("Invalid context size: {}. Try with a context size equal to or smaller than the largest size provided in the \"load\" instruction.", windowsize);
                }

                let output: String = generate(gen_mode, start_word, windowsize, &markov_chain);

                println!("{}", output);
            }
            "quit" => break,
            _ => {
                println!("Unrecognized command: {}", command);
            }
        }
    }
}

fn populate(
    text: &Vec<&str>,
    index: usize,
    context_depth: usize,
    data: &mut HashMap<String, MarkovNode>,
) {
    // Recursively populate the Markov chain

    if context_depth == 0 || index >= text.len() {
        return;
    }

    let curr_word = text[index].to_string().replace(
        &[
            '(', ')', ',', '.', '?', '!', ':', ';', '\'', '\"', '\\', '/', ' ',
        ],
        "",
    );

    if let Some(next_node) = data.get_mut(&curr_word) {
        next_node.freq = next_node.freq + 1;
        populate(text, index + 1, context_depth - 1, &mut next_node.next);
    } else {
        let tmp_node = MarkovNode {
            freq: 1,
            next: HashMap::new(),
        };
        data.insert(curr_word.clone(), tmp_node);
        if let Some(next_node) = data.get_mut(&curr_word) {
            populate(text, index + 1, context_depth - 1, &mut next_node.next);
        } else {
            println!("Something went wrong.");
            std::process::exit(1);
        }
    }
}

fn generate(
    mode: &str,
    first_word: &str,
    context_depth: usize,
    data: &HashMap<String, MarkovNode>,
) -> String {
    if data.get(first_word).is_none() {
        println!(
            "The word: {} does not exist in the text. Try something else.",
            first_word
        );
        std::process::exit(1);
    }

    let mut gen_string = String::from(first_word);
    let mut context_counter = context_depth;
    let mut key_val = String::from(first_word);

    match mode {
        "most_common" => loop {
            if context_counter == 0 {
                break;
            }
            let most_common_next: String = find_most_common(&data[&key_val].next);
            gen_string.push_str(" ");
            gen_string.push_str(&most_common_next);
            key_val = most_common_next;
            context_counter = context_counter - 1;
        },
        "random" => loop {
            if context_counter == 0 {
                break;
            }
            let rand_word: &String = data[&key_val].next.keys().last().unwrap();
            gen_string.push_str(" ");
            gen_string.push_str(rand_word);
            key_val = rand_word.to_string();
            context_counter = context_counter - 1;
        }
        _ => {
            println!("Unrecognized mode: {}", mode);
            std::process::exit(1);
        }
    }

    gen_string
}

fn find_most_common(data: &HashMap<String, MarkovNode>) -> String {
    let mut highest_freq: u16 = 0;
    let mut return_key: String = String::new();
    for key in data.keys() {
        if data[key].freq > highest_freq {
            highest_freq = data[key].freq;
            return_key = key.to_string();
        }
    }

    return_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate() {
        let test_text = fs::read_to_string("texts/unit_test_populate.txt")
            .expect(&format!("Unable to read unit_test_populate.txt"));
        let mut markov_chain: HashMap<String, MarkovNode> = HashMap::new();
        let windowsize = 5;

        let words: Vec<&str> = test_text.split(" ").collect();

        for (i, _) in words.iter().enumerate() {
            populate(&words, i, windowsize, &mut markov_chain);
        }

        let expected_word = String::from("test");

        let word = find_most_common(&markov_chain);

        assert_eq!(word, expected_word);
    }
}
