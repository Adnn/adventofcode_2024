use std::env;
use std::fs;

//enum Level {
//    First,
//    Previous(i32),
//}
//
//impl Level {
//    fn compare(& mut self, other: i32) -> bool
//    {
//        match self {
//            Level::First => true,
//            Leve::Previous => (self.0 - other) <
//        }
//    }
//}

enum Direction {
    Inc,
    Dec,
}

struct Report {
    values: Vec<i32>,
}

impl Report {

    fn push(& mut self, level: i32) -> () {
        self.values.push(level);
    }

    fn safe(& self) -> bool {
        let dir = if self.values[0] < self.values[1] {Direction::Inc}
                  else {Direction::Dec};

        let mut safe = true;
        let v = &self.values;
        for idx in 0..=v.len()-2 {
            let diff = v[idx + 1] - v[idx];
            match dir {
                Direction::Inc => safe = safe && diff >= 1 && diff < 4,
                Direction::Dec => safe = safe && diff <= -1 && diff > -4,
            }
        }
        return safe;
    }

    fn dampen(&self) -> Vec<Self> {
        let mut result = Vec::new();
        for idx in 0..self.values.len() {
            let mut sub = Report {
                values: self.values.clone(),
            };
            sub.values.remove(idx);
            result.push(sub);
        }
        result
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &str = &args[1];
    println!("Processing inputs from '{input_path}'");

    let content = fs::read_to_string(input_path)
        .expect("Could not read file content.");

    let mut safe = 0;
    for line in content.lines() {
        // Report (line)
        let mut report = Report {
            values: Vec::new(),
        };
        for level in line.split(" ") {
            // Levels
            report.push(level.parse().expect("Cannot parse to int"))
        }

        if report.safe() {
            safe = safe + 1;
        }
        else {
            for sub_report in report.dampen() {
                if sub_report.safe()
                {
                    safe = safe + 1;
                    break;
                }
            }
        }
    }

    println!("Safe reports: {safe}");
}
