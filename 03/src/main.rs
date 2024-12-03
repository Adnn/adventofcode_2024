use std::env;
use std::fs;


fn path_from_args() -> String {
    //let args: Vec<String> = env::args().collect();
    //match args.get(1) {
    //    None => panic!("Requires a file path as 1st argument."),
    //    Some(p) => *p,
    //}
    return env::args().nth(1).expect("Requires a file path as 1st argument.");
}


fn read_input(input_path: &str) -> String {
    println!("Read inputs from '{input_path}'");
    let content = fs::read_to_string(input_path)
        .expect("Could not read file content.");
    return content;
}

//struct Slicer {
//    text: &String,
//    offset: usize,
//}
//
//impl Slicer {
//    fn get(&self) -> &str {
//        self.text[self.offset..]
//    }
//}


fn extract_number(content: &str) -> (Option<u32>, usize) {
    let mut num_length = 0;
    while content[num_length..].chars().nth(0).expect("Still believe").is_numeric() {
        num_length += 1;
    }

    if num_length >= 1 && num_length <= 3 {
        (Some(content[..num_length].parse().expect("Should be parseable really")), num_length)
    }
    else {
        (None, num_length)
    }
}


fn main() {
    let content = read_input(&path_from_args());

    // Let's write an ad-hoc automata
    let mut accum: u32 = 0;
    let mut offset: usize = 0;
    while offset < content.len() {
        if content[offset..].starts_with("mul(") {
            offset += 4;

            let (operand, advance) = extract_number(&content[offset..]);
            offset += advance;
            if let Some(left) = operand {
                if content[offset..].starts_with(",") {
                    offset += 1;
                    let (operand, advance) = extract_number(&content[offset..]);
                    offset += advance;
                    if let Some(right) = operand {
                        if content[offset..].starts_with(")") {
                            println!("Mul ({left}, {right})");
                            accum += left * right;
                        }
                    }
                }
            }
        }
        else {
            offset += 1;
        }
    }

    println!("Result: {accum}");
}
