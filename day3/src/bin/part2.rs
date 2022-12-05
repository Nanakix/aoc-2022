
use std::collections::hash_set::Intersection;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use::itertools::Itertools;

fn main() {
    let file_path = "input";
    let mut group: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        let mut cpt: u32 = 0;
            for three in &lines.into_iter().chunks(3) { // parse lines 3 by 3
                for i in three {
                    group.push(i.unwrap());
                }
                cpt += check_badges(&group);
                group.clear();
            }
            print!("total badge score: {}", cpt);
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

fn check_badges(group: &Vec<String>) -> u32 {
    // find common char between 3 bags (@param group: vector containing the Strings of the 3 bags)
    let res = find_priority(&group[0].chars().collect::<HashSet<char>>(), &group[1].chars().collect::<HashSet<char>>(), &group[2].chars().collect::<HashSet<char>>());
    return res as u32;
}

fn find_priority(one: &HashSet<char>, two: &HashSet<char>, three: &HashSet<char>) -> u8{
    // now that we have a sorted bag, let's find the common char for the priority
    let priority: Intersection<char, std::collections::hash_map::RandomState> = one.intersection(two);
    let v: Vec<&char> = priority.filter(|elem| three.contains(elem)).collect();
    println!("{:?}", &v);
    let p: char = **v.last().unwrap();
    match p {
        'a'..='z' => (p as u8) - b'a' + 1,
        'A'..='Z' => (p as u8) - b'A' + 27,
        _ => 0
    }
}