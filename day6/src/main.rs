use std::collections::{ HashSet};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = "input";
    if let Ok(lines) = read_lines(file_path) {
        parse_uniques(lines.flatten().collect());
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

fn parse_uniques(s: String) -> usize {
    let mut res = 0;
    let mut found: bool = false;
    for (i, window) in s.into_bytes().windows(4).enumerate() {
        let mut hs:HashSet<&u8> = HashSet::new();
        if !found {
            for c in window {
                hs.insert(&c);
            }
            if hs.len() == 4 {
                println!("{}, {:?}",i, &hs);
                res = i+4; // because first window with 4 unique elements will be at (position of the fourth unique char in a row) -4
                found = true;
                println!("{}",res);
            }
        }
            hs.clear();
    }
    res
}

#[test]
fn test_parse() {
    let expected = 5;
    let res = parse_uniques("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
    assert_eq!(expected, res);
}
