use std::env;
use std::fs;


fn path_from_args() -> String {
    // Note: apparently, there is no way to return a value from a Vec
    // even though it is "expriging".
    //let args: Vec<String> = env::args().collect();
    //match args.get(1) {
    //    None => panic!("Requires a file path as 1st argument."),
    //    Some(p) => *p,
    //}
    return env::args().nth(1).expect("Requires a file path as 1st argument.");
}


fn read_input(input_path: &str) -> String {
    println!("Read inputs from '{input_path}'");
    fs::read_to_string(input_path)
        .expect("Could not read file content.")
}


#[derive(Debug)]
struct WordSearch {
    input: Vec<char>,
    width: usize,
    height: usize,
}


impl WordSearch {
    fn make(input: & mut String) -> WordSearch {
        let width = input.lines().nth(0).expect("no line").len();
        // This is probably bad, reading the whole string
        let height = input.lines().count();
        // remove newlines for both usual conventions
        let replaced = input.replace("\r\n", "").replace("\n", "")
            .chars().collect();
        WordSearch {
            input: replaced,
            width,
            height,
        }
    }

    fn search(&self, idx: usize, pattern: &str) -> u32 {
        let x0 = idx % self.width;
        let x1 = self.width - x0 - 1;
        let y0 = idx / self.height;
        let y1 = self.height - y0 - 1;
        //println!("for idx '{idx}', ({x0}, {x1}), ({y0}, {y1})");

        let mut count = 0;
        if x0 >= pattern.len() && match_pattern(&self.input, pattern, idx, -1) {
            count += 1;
        }
        if x1 >= pattern.len() && match_pattern(&self.input, pattern, idx, 1) {
            count += 1;
        }
        if y0 >= pattern.len() && match_pattern(&self.input, pattern, idx, -(self.width as i32)) {
            count += 1;
        }
        if y1 >= pattern.len() && match_pattern(&self.input, pattern, idx, self.width as i32) {
            count += 1;
        }
        if x0 >= pattern.len() && y0 >= pattern.len() && match_pattern(&self.input, pattern, idx, -1-(self.width as i32)) {
            count += 1;
        }
        if x0 >= pattern.len() && y1 >= pattern.len() && match_pattern(&self.input, pattern, idx, -1+(self.width as i32)) {
            count += 1;
        }
        if x1 >= pattern.len() && y0 >= pattern.len() && match_pattern(&self.input, pattern, idx, 1-(self.width as i32)) {
            count += 1;
        }
        if x1 >= pattern.len() && y1 >= pattern.len() && match_pattern(&self.input, pattern, idx, 1+(self.width as i32)) {
            count += 1;
        }
        count
    }
}

fn match_pattern(text: &Vec<char>, pattern: &str, start: usize, inc: i32) -> bool {
    for (i, c) in pattern.chars().enumerate() {
        //let position: i32 = start as i32 + (inc as i32 * i);
        let position: i32 = start as i32 + (i as i32 + 1) * inc;
        if ! (text[position as usize] == c) {
            return false;
        }
    }
    true
}


fn main() {
    let mut content = read_input(&path_from_args());
    //println!("Content: {content}");


    let mut total = 0;
    let puzzle = WordSearch::make(&mut content);
    for (idx, c) in puzzle.input.iter().enumerate() {
        if *c == 'X' {
            total += puzzle.search(idx, "MAS");
        }
    }
    println!("Total occurrences of 'XMAS' in the puzzle ({}, {}): {total}",
        puzzle.width,
        puzzle.height);
}
