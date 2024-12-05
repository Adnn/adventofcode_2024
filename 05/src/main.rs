use std::env;
use std::fs;
use std::collections::HashMap;


fn path_from_args() -> String {
    return env::args().nth(1).expect("Requires a file path as 1st argument.");
}


fn read_input(input_path: &str) -> String {
    println!("Read inputs from '{input_path}'");
    fs::read_to_string(input_path)
        .expect("Could not read file content.")
}


type Page = u32;


#[derive(Debug)]
struct Rule {
    page: Page,
    before: Vec<Page>,
}


impl Rule {
    fn must_occur_before(&self, page: Page) -> bool {
        self.before.contains(&page)
    }
}

fn populate_rules(input: &str) -> (HashMap<Page, Rule>, u32) {
    let mut rules: HashMap<Page, Rule> = HashMap::new();
    let mut lines = input.lines();
    let mut processed = 0;
    for line in lines {
        processed += 1;
        if line.len() == 0 {
            // An empty line means we are done with the rules definition section
            break;
        }
        else {
            let mut sp = line.split('|');
            let page: Page = sp.next().unwrap().parse().unwrap();
            let follower: Page = sp.next().unwrap().parse().unwrap();

            let rule = rules.entry(page).or_insert(
                Rule {
                    page,
                    before: Vec::new()
                });
            rule.before.push(follower);
        }
    }
    (rules, processed)
}


fn has_duplicates(mut pages: Vec<Page>) -> bool {
    pages.sort();
    let pre = pages.len();
    pages.dedup();
    return pages.len() != pre;
}

// Returns a vector whose indices are page numbers, and values are numbers of followers
fn produce_ordering(map: & HashMap<Page, Rule>) -> Vec<Page> {
    let mut ordering: Vec<Page> = Vec::new();
    // We know from observing the input all page numbers are < 100
    ordering.resize(100, 0);

    for (page, rule) in map {
        assert!(*page == rule.page);
        assert!(!has_duplicates(rule.before.clone()));
        ordering[*page as usize] = rule.before.len() as Page;
    }
    ordering
}


fn middle_page_failed(update: &str, ordering: &Vec<Page>) -> Option<Page>
{
    let update_pages: Vec<Page> =
        update.split(',').map(str::parse).map(Result::unwrap).collect();
    let mut followers_prev = ordering[update_pages[0] as usize];
    for page in &update_pages[1..] {
        let followers = ordering[*page as usize];
        if followers >= followers_prev {
            return None;
        }
        else {
            followers_prev = followers;
        }
    }
    Some(update_pages[update_pages.len() / 2])
}


enum Sorted<T> {
    Direct(T),
    Recurse(T),
}


fn middle_page(update_pages: &Vec<Page>, map: &HashMap<Page, Rule>) -> Sorted<Page>
{
    let mut pages_before = vec![update_pages[0]];
    for (this_idx, page) in update_pages[1..].iter().enumerate() {
        for (prev_idx, prev) in pages_before.iter().enumerate() {
            if let Some(rule) = map.get(page) {
                if rule.must_occur_before(*prev) {
                    // Insert the current value before the value it must occure before
                    let mut pages_fixed = update_pages.clone();
                    let this_value = pages_fixed.remove(this_idx+1);
                    pages_fixed.insert(prev_idx, this_value);

                    // There is for sure something idiomatic to replace this match
                    match middle_page(&pages_fixed, map) {
                        Sorted::Direct(page) => return Sorted::Recurse(page),
                        Sorted::Recurse(page) => return Sorted::Recurse(page),
                    }

                }
            }
        }
        pages_before.push(*page);
    }

    Sorted::Direct(update_pages[update_pages.len() / 2])
}


fn main() {
    let content = read_input(&path_from_args());

    let (hash_rules, mut lines_processed) = populate_rules(&content);
    println!("Hash:\n{hash_rules:?}");

    let ordering = produce_ordering(&hash_rules);
    println!("Ordering:\n{ordering:?}");

    let mut accum_direct = 0;
    let mut accum_recurse = 0;
    for line in content.lines() {
        // I could not find a way to return the iterators from remaining position
        if lines_processed != 0 {
            lines_processed -= 1;
            continue;
        }

        println!("Handle line: '{line}'.");
        //if let Some(middle) = middle_page_failed(&line, &ordering) {
        let update_pages: Vec<Page> =
            line.split(',').map(str::parse).map(Result::unwrap).collect();

        match middle_page(&update_pages, &hash_rules) {
            Sorted::Direct(midpage) => accum_direct += midpage,
            Sorted::Recurse(midpage) => accum_recurse += midpage,
        }
    }

    println!(r#"Accumulated middle pages:
        * direct: {accum_direct}
        * fixed: {accum_recurse}"#);
}
