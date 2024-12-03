use std::env;
use std::fs;


fn main()
{
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    //dbg!(args);
    println!("Reading input from '{input_path}'");

    let content = fs::read_to_string(input_path)
        // Note: no workito, need to understand string formatting
        .expect("Unable to read '{input_path}'");

   // println!("Content:\n{content}");

    let mut left:  Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // What is lines return type?
    let lines = content.lines();
    for line in lines
    {
        let numbers: Vec<String> = line.to_string().split_whitespace()
            .map(str::to_string).collect();
        left.push(numbers[0].parse().unwrap());
        right.push(numbers[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut accu = 0;
    let mut idx = 0;
    while idx != left.len()
    {
        accu += (left[idx] - right[idx]).abs();
        idx += 1;
    }

    println!("Accumulation of difference is {accu}");

    accu = 0;
    idx = 0;
    let mut right_idx = 0;
    while idx != left.len()
    {
        let value = left[idx];
        while right_idx != right.len() && right[right_idx] < value
        {
            right_idx += 1;
        }
        let mut right_occurrences = 0;
        while right_idx != right.len() && right[right_idx] == value
        {
            right_occurrences += 1;
            right_idx += 1;
        }
        let mut left_occurrences = 0;
        while idx != left.len() && left[idx] == value
        {
            left_occurrences += 1;
            idx += 1;
        }
        accu += value * right_occurrences * left_occurrences;
    }

    println!("Similarity score is {accu}");
}
