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


fn main() {
    let content = read_input(&path_from_args());

    println!("Content: {content}");
}
