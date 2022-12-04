
use std::collections::hash_set::Intersection;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let file_path = "input";

    if let Ok(lines) = read_lines(file_path) {
        let mut total_priority: u32 = 0;
        for line in lines {
            match line {
                Ok(tmp) => {
                    total_priority += check_bag(&tmp) as u32; // p1
                }
                Err(_e) => println!("Error reading line")
            }
        }
        print!("total priority: {}", total_priority); // p1
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}       

fn check_bag(bag: &str) -> u8 {
    // reminder: I checked the input, size of bag is always an even number
    // each bag has two compartments of equal size
    let compart_one: HashSet<char> = bag[..bag.len()/2].chars().collect();
    let compart_two: HashSet<char> = bag[bag.len()/2..].chars().collect();
    find_priority(&compart_one, &compart_two)
}

fn find_priority(one: &HashSet<char>, two: &HashSet<char>) -> u8{
    // now that we have a sorted bag, let's find the common char for the priority
    let priority: Intersection<char, std::collections::hash_map::RandomState> = one.intersection(two);
    let p: char = priority.last().unwrap().clone(); // awful conversion T_T

    match p {
        'a'..='z' => (p as u8) - b'a' + 1,
        'A'..='Z' => (p as u8) - b'A' + 27,
        _ => 0
    }
}