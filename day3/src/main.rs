
use std::collections::hash_set::Intersection;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let file_path = "input";

    if let Ok(lines) = read_lines(file_path) {
        let mut total_priority = 0;
        for line in lines {
            match line {
                Ok(tmp) => {
                    total_priority += check_bag(&tmp);
                }
                Err(_e) => println!("Error reading line")
            }
        }
        print!("total priority: {}", total_priority);
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

fn check_bag(tmp: &str) -> u32 {
    // reminder: I checked the input, size of bag is always an even number
    // each bag has two compartments of equal size
    let compartment_size: usize = tmp.len()/2;

    let mut compart_one: HashSet<char> = HashSet::new();
    let mut compart_two: HashSet<char> = HashSet::new();


    let mut cpt: u32 = 0;
    for c in tmp.chars() { // let's fill the compartments
        if cpt < compartment_size.try_into().unwrap() { // unwrap conversion from usize to u32
            cpt += 1;
            compart_one.insert(c);
        }
        else {
            compart_two.insert(c);
        }
    } // I did not find an elegant way to do this, forgive me senpai
    find_priority(&compart_one, &compart_two)
}

fn find_priority(one: &HashSet<char>, two: &HashSet<char>) -> u32{
    // now that we have a sorted bag, let's find the common char for the priority
    let priority: Intersection<char, std::collections::hash_map::RandomState> = one.intersection(two);
    let p: char = priority.last().unwrap().clone(); // awful conversion T_T
    let res: u32 =  p as u32;

    // priority: a = 1, .. z = 26, A=27, .. Z =52 
    if p.is_lowercase() {
        res - 96 // ascii table starts lower case alphabet @97
    }
    else {
        res - 64 + 26 // ascii table starts upper case alphabet @65
    }
}