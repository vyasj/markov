# Markov Chain Sentence Generator

I was bored, and needed a project to help me learn Rust a bit better. This is my attempt.

Train the markov chain on any block of text via a `.txt` file and generate a sentence of a given length by either picking the most common next word or randomly picking from the set of words that followed in the original text.

## Functionality

Currently only two commands are supported:

`load`: expects two more arguments, `<text_file_name> <context_size>`. Example: `load beemovie.txt 10`, reads the contents of `beemovie.txt` and trains the markov chain on its contents with a contents depth of 10 words.

`generate`: expects three more arguments, `<mode> <starting_word> <length>`. Example: `generate most_common honey 7`, generates a sentence using the most common following word starting with the word "honey", up to 7 more words for a sentence of 8 words.

### Modes

There are currently two modes for generation.

- `most_common` exclusively uses the most commonly occurring next word at each level to generate a sentence.
- `random` randomly picks from the list of next possible words to generate a sentence.

## Usage

`cargo run <command_file>` where `<command_file>` is a `.txt` file containing the commands to run, separated by line. See the `test.txt` files for an example.