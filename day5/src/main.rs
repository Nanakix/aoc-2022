    // use core::num::dec2flt::parse;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = "input";
    // find the line of the numbers
    // let mut nb_stacks = 0;
    // if let Ok(lines) = read_lines(file_path) {
    //     let l : String = lines.flat_map(|l| l).find(|line| find_numbers(&line)).unwrap();
    //     nb_stacks = l.trim().chars().last().unwrap_or_default().to_digit(10).unwrap_or_default();
    //     // let nb2 = l.split_ascii_whitespace().last().unwrap_or_default().trim().parse::<u32>().unwrap_or_default();
    // }

    // // 2nd reading: fill the vectors
    let mut v : Vec<VecDeque<u8>> = Vec::new();
    v.resize(9, VecDeque::with_capacity(8));
    // construct all stacks
    if let Ok(lines) = read_lines(file_path) {
        for (index, l) in lines.flat_map(|l| l).enumerate() {
            if index < 9 {
                for (i, chunk) in l.as_bytes().chunks(4).enumerate() {
                    if chunk[1] != 32 {
                        v[i].push_front(chunk[1]);
                    }
                }
            }
            else {
                for vdq in &v {
                    print!("{:?} ", (&vdq.back().unwrap()));
                }
                println!();
                println!("{:?}", &v);
                let load: Vec<u8>;
                load = l.split(" ").skip(1).step_by(2).flat_map(|w| w.parse::<u8>()).collect();
                for _ in 0..load[0] {
                    let tmp = v[(load[1]-1) as usize].pop_back().unwrap_or_default();
                    v[(load[2]-1) as usize].push_back(tmp);
                }
                println!();
                println!("{:?}", &v);
                println!();
            }
        }
        for vdq in &v {
            print!("{:?} ", (&vdq.back().unwrap())); // this prints the result as u8 need to convert it to char
        }
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

// fn find_numbers(line: &str) -> bool{

//     // v2
//     line.chars().any(|c| return c.is_digit(10))
// }
    
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
