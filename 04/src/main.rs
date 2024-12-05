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

    fn search_8directions(&self, idx: usize, pattern: &str) -> u32 {
        // compute how much room is left on the rectangular input from idx.
        // left, right, top, bottom.
        let x0 = idx % self.width;
        let x1 = self.width - x0 - 1;
        let y0 = idx / self.height;
        let y1 = self.height - y0 - 1;
        //println!("for idx '{idx}', ({x0}, {x1}), ({y0}, {y1})");

        let min_room = pattern.len() - 1;
        let mut count = 0;
        if x0 >= min_room && match_pattern(&self.input, pattern, idx, -1) {
            count += 1;
        }
        if x1 >= min_room && match_pattern(&self.input, pattern, idx, 1) {
            count += 1;
        }
        if y0 >= min_room && match_pattern(&self.input, pattern, idx, -(self.width as i32)) {
            count += 1;
        }
        if y1 >= min_room && match_pattern(&self.input, pattern, idx, self.width as i32) {
            count += 1;
        }
        if x0 >= min_room && y0 >= min_room && match_pattern(&self.input, pattern, idx, -1-(self.width as i32)) {
            count += 1;
        }
        if x0 >= min_room && y1 >= min_room && match_pattern(&self.input, pattern, idx, -1+(self.width as i32)) {
            count += 1;
        }
        if x1 >= min_room && y0 >= min_room && match_pattern(&self.input, pattern, idx, 1-(self.width as i32)) {
            count += 1;
        }
        if x1 >= min_room && y1 >= min_room && match_pattern(&self.input, pattern, idx, 1+(self.width as i32)) {
            count += 1;
        }
        count
    }

    fn search_diagonals(&self, idx: usize, pattern: &str) -> u32 {
        // compute how much room is left on the rectangular input from idx.
        // left, right, top, bottom.
        let x0 = idx % self.width;
        let x1 = self.width - x0 - 1;
        let y0 = idx / self.height;
        let y1 = self.height - y0 - 1;

        let mut count = 0;
        // If there is at least 1 position on each direction (to fit the X)
        if x0 >= 1 && x1 >= 1 && y0 >= 1 && y1 >= 1 {
            let diag_1_inc = 1 + self.width as i32;
            let top_left = idx as i32 - diag_1_inc as i32;
            let bottom_right = idx as i32 + diag_1_inc as i32;
            let diag_2_inc = 1 - self.width as i32;
            let bottom_left = idx as i32 - diag_2_inc as i32;
            let top_right = idx as i32 + diag_2_inc as i32;
            if (match_pattern(&self.input, pattern, top_left as usize, diag_1_inc)
                || match_pattern(&self.input, pattern, bottom_right as usize, -diag_1_inc))
                &&
                (match_pattern(&self.input, pattern, bottom_left as usize, diag_2_inc)
                || match_pattern(&self.input, pattern, top_right as usize, -diag_2_inc)) {
                count += 1;
            }
        }
        count
    }
}

fn match_pattern(text: &Vec<char>, pattern: &str, start: usize, inc: i32) -> bool {
    for (i, c) in pattern.chars().enumerate() {
        //let position: i32 = start as i32 + (inc as i32 * i);
        let position: i32 = start as i32 + (i as i32) * inc;
        if text[position as usize] != c {
            return false;
        }
    }
    true
}


fn main() {
    let mut content = read_input(&path_from_args());
    //println!("Content: {content}");


    let mut total_1 = 0;
    let mut total_2 = 0;
    let puzzle = WordSearch::make(&mut content);
    for (idx, c) in puzzle.input.iter().enumerate() {
        if *c == 'X' {
            // Wastefully test for X a second time.
            total_1 += puzzle.search_8directions(idx, "XMAS");
        }
        else if *c == 'A' {
            total_2 += puzzle.search_diagonals(idx, "MAS");
        }
    }
    println!(
    r#"In the puzzle ({}, {})
    - occurrences of "XMAS": {total_1}
    - occurrences of "MAS" in a X: {total_2}"#,
        puzzle.width,
        puzzle.height);
}
