use std::collections::{HashSet};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let file_path = "input";
    let mut head: (i32, i32) = (0,0); // (X, Y) of the head of the rope
    let mut tail: (i32, i32) = (0,0);
    let mut visited_pos: HashSet<(i32,i32)> = HashSet::new();
    // let mut directions: HashSet<String, u32> = HashSet::new();
    
    if let Ok(lines) = read_lines(file_path) {
        for l in lines.flatten() {
            parse_ropes(l, &mut head, &mut tail, &mut visited_pos);
        }
    }
        println!("res: {}", visited_pos.iter().len() );// p1: number of visited pos by the tail
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn check_positions(head: &mut (i32, i32), tail: &mut (i32, i32), visited_pos: &mut HashSet<(i32,i32)>) {
    if ((head.0- tail.0).abs() > 1 && (head.1 - tail.1).abs() > 0) || ((head.0- tail.0).abs() > 0 && (head.1 - tail.1).abs() > 1) {
        // tester le sens enfin
        if head.0 > tail.0 { //RIGHT
            tail.0 += 1;
        }
        else { // LEFT
            tail.0 -= 1;
        }
        if head.1 > tail.1 { // UP
            tail.1 += 1;
        }
        else { // DOWN
                tail.1 -= 1;
            }
        visited_pos.insert(tail.clone());
    }
    if (head.0 - tail.0).abs() > 1 {
        if head.0 > tail.0 { //RIGHT
            tail.0 += 1;
        }
        else { // LEFT
            tail.0 -= 1;
        }
        println!("MOVE head: {:?} TAIL: {:?}", &head, &tail);  
    }
    visited_pos.insert(tail.clone());
    if (head.1 - tail.1).abs() > 1 {
        if head.1 > tail.1 { // UP
            tail.1 += 1;
        }
        else { // DOWN
            tail.1 -= 1;
        }
        println!("MOVE head: {:?} TAIL: {:?}", &head, &tail);  
    }
    visited_pos.insert(tail.clone());
}

fn parse_ropes(s: String, head: &mut (i32, i32), tail: &mut (i32, i32), visited_pos: &mut HashSet<(i32,i32)>) {
    let mut sp: Vec<&str> = s.split(" ").collect();
    let nb: u32 = sp[1].parse().unwrap(); // if it's a number
    // up down Y, right left X
    visited_pos.insert((0,0));
    match sp[0] { // direction
        "U" => {
            for _ in 0..nb {
                head.1 += 1;
                println!("Up: HEAD: {:?} tail: {:?}", &head, &tail);  
                check_positions(head, tail, visited_pos);
            }
        },
        "D" => { 
            for _ in 0..nb {
                head.1 -= 1;
                println!("Down: HEAD: {:?} tail: {:?}", &head, &tail);  
                check_positions(head, tail, visited_pos);
            }
        },
        "L" => {
            for _ in 0..nb {
                head.0 -= 1;
                println!("Left: HEAD: {:?} tail: {:?}", &head, &tail);  
                check_positions(head, tail, visited_pos);
            }
        },
        "R" => { 
            for _ in 0..nb {
                head.0 += 1;
                println!("Right: HEAD: {:?} tail: {:?}", &head, &tail);  
                check_positions(head, tail, visited_pos);
            }
        },
        _ => {
            panic!("wtf");
        }
    }    
}   

#[test]
fn test_parse() {
    let expected: u32 = 13;
    let mut head: (i32, i32) = (0,0);
    let mut tail: (i32, i32) = (0,0);
    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();
    if let Ok(lines) = read_lines("test") {
        for l in lines.flatten() {
            parse_ropes(l, &mut head, &mut tail, &mut visited_pos)
        }
    }
    println!("res: {}", visited_pos.iter().len() );// p1: number of visited pos by the tail
    let res = visited_pos.iter().len() as u32;
    assert_eq!(expected, res);
}
