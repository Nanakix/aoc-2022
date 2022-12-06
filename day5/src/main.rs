use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = "test";
    // find the line of the numbers
    let mut nb_stacks = 0;
    if let Ok(lines) = read_lines(file_path) {
        let l : String = lines.flat_map(|l| l).find(|line| find_numbers(&line)).unwrap();
        nb_stacks = l.trim().chars().last().unwrap_or_default().to_digit(10).unwrap_or_default();
        // let nb2 = l.split_ascii_whitespace().last().unwrap_or_default().trim().parse::<u32>().unwrap_or_default();
    }
    
    // let v : Vec<Vec<char>> = Vec::new();
    // v.resize(9, vec![])
    // construct all stacks
    // if let Ok(lines) = read_lines(file_path) {
        // for four in &lines.into_iter().chunks(4) {

        // }
    // }
    //     for (i, line) in lines.enumerate() {
    //         match line {
    //             Ok(tmp) => {
    //                 // 
    //             },
    //             Err(_e) => println!("Error reading line")
    //         }
    //     }
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn find_numbers(line: &str) -> bool{

    // v2
    line.chars().any(|c| return c.is_digit(10))
    
    // for c in line.chars()
    //     if c.is_digit(10)
    //         return true;
    // v1
    //line.chars().map(|c|  return c.is_digit(10)).find(|&b| b).unwrap()
    
    // v de merde
    // for c in line.chars() {
    //     let tmp = c.to_digit(10);
    //     match tmp {
    //         Some(d) => if d == 1 {
    //             return true;
    //         },
    //         N
    //     }
    // }
    // false
}
