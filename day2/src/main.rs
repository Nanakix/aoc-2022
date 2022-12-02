

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn main() {

    let file_path = "input";

    if let Ok(lines) = read_lines(file_path) {
        let mut score: u32 = 0;
        for line in lines {
            match line {
                Ok(tmp) => { 
                    if tmp.eq("") { 
                        // match is over, next opponent
                    }
                    else {
                        let round = tmp.split_once(" ").unwrap();
                        score += tatakau(round.0, round.1) as u32;
                    }
                }
            Err(_e) => println!("Error reading line")
            }
        }
        print!("final score: {}", score);
    }
}

fn tatakau(uke: &str, tori: &str) -> u32 {
// A Rock, B Paper, C Scissors
// X Rock, Y Paper, Z Scissors
// PART 1    
    match uke {
    //     "A" =>  match tori {
    //                 "X" => 1+3,
    //                 "Y" => 2+6,
    //                 "Z" => 3,
    //                 _ => panic!("wtf !s dat r0und m8"),
    //         },
    //     "B" =>  match tori {
    //             "X" => 1,
    //             "Y" => 2+3,
    //             "Z" => 3+6,
    //             _ => panic!("wtf !s dat r0und m8"),
    //         },
    //     "C" => match tori {
    //             "X" => 1+6,
    //             "Y" => 2,
    //             "Z" => 3+3,
    //             _ => panic!("wtf !s dat r0und m8"),
    //         },
    //     _ => panic!("wtf !s dat r0und m8: {}", uke),
        // X loose, Y draw, Z win
        "A" =>  match tori { // against rock
                "X" => 3+0, // loss = scissors
                "Y" => 1+3, // draw = rock
                "Z" => 2+6, // win = paper
                _ => panic!("wtf !s dat r0und m8"),
        },
        "B" =>  match tori { // against paper
            "X" => 1+0, // loss = rock
            "Y" => 2+3, // draw = paper
            "Z" => 3+6, // win = scissors
            _ => panic!("wtf !s dat r0und m8"),
        },
        "C" => match tori { // against scissors
            "X" => 2+0, // loss = paper
            "Y" => 3+3, // draw = scissors
            "Z" => 1+6, // win = rock
            _ => panic!("wtf !s dat r0und m8"),
        },
        _ => panic!("wtf !s dat r0und m8: {}", uke),
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