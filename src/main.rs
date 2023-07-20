use std::io::BufRead;

use wordle::wordle::Wordle;

fn main() {
    let mut wordle = Wordle::from("/home/alefir/.local/share/wordle_words");
    let stdin = std::io::stdin();
    let mut input = String::new();

    for guess in 1..=6 {
        println!("({}): {} words", guess, wordle.len());
        input.clear();
        stdin
            .lock()
            .read_line(&mut input)
            .expect("Failed to read input");

        wordle.update(input.trim()).expect("Failed to parse input");

        if wordle.len() < 400 {
            for word in wordle.words() {
                println!("{}", word);
            }
        }
    }
}
