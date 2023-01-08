use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let mut cycle_count: u32 = 0;
    let mut reg_x: i32 = 1;
    let mut reg_x_snapshots: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input") {
        for l in lines.flatten() {
            parse_instr(l,  &mut cycle_count, &mut reg_x, &mut reg_x_snapshots);
        }
    }
    println!("res: {:?}", reg_x_snapshots );// p1: number of visited pos by the tail
    // let res = visited_pos.iter().len() as u32;
    println!("{}, {}", reg_x_snapshots[19], 20 * reg_x_snapshots[19]);
    println!("{}, {}", reg_x_snapshots[59], 60 * reg_x_snapshots[59]);
    println!("{}, {}", reg_x_snapshots[99], 100 * reg_x_snapshots[99]);
    println!("{}, {}", reg_x_snapshots[139], 140 * reg_x_snapshots[139]);
    println!("{}, {}", reg_x_snapshots[179], 180 * reg_x_snapshots[179]);
    println!("{}, {}", reg_x_snapshots[219], 220 * reg_x_snapshots[219]);
    let res: i32 = 20 * reg_x_snapshots[19] + 60 * reg_x_snapshots[59] + 100 * reg_x_snapshots[99] + 140* reg_x_snapshots[139] + 180* reg_x_snapshots[179] + 220* reg_x_snapshots[219];
    println!("res: {}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn parse_instr(s: String, cycle_count: &mut u32, reg_x: &mut i32,  reg_x_snapshots: &mut Vec<i32> ) {
    let sp: Vec<&str> = s.split(" ").collect();
    match sp.len() {
        1 => {
            run_cycle(sp[0], cycle_count, &reg_x, reg_x_snapshots)
        },
        2 => {
            match sp[0] {
                "addx" => {
                    run_cycle(sp[0], cycle_count, &reg_x, reg_x_snapshots);
                    let nb: i32 = sp[1].parse().unwrap(); // if it's a number
                    *reg_x += nb;
                },
                _ => {
                    panic!("unknown instruction");
                }
            }
        },
        _ => {
            panic!("undefined behavior");
        }
    }
    
}   

fn run_cycle (op: &str, cycle_count: &mut u32, reg_x: &i32, reg_x_snapshots: &mut Vec<i32>) {
    match op {
        "noop" => {
            reg_x_snapshots.push(*reg_x);
            *cycle_count += 1;
        },
        "addx" => {
            reg_x_snapshots.push(*reg_x);
            reg_x_snapshots.push(*reg_x);
            *cycle_count += 2;
        },
        _ => println!("unknow cycle increment")
    }
}

#[test]
fn test_parse() {
    let expected: i32 = 3;
    let mut cycle_count: u32 = 0;
    let mut reg_x: i32 = 1;
    let mut reg_x_snapshots: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("test") {
        for l in lines.flatten() {
            parse_instr(l,  &mut cycle_count, &mut reg_x, &mut reg_x_snapshots);
        }
    }
    println!("res: {:?}", reg_x_snapshots );// p1: number of visited pos by the tail
    // let res = visited_pos.iter().len() as u32;
    let res: i32 = reg_x_snapshots[3] + reg_x_snapshots[reg_x_snapshots.len() - 1];
    assert_eq!(expected, res);
}

#[test]
fn test_parse2() {
    let expected: i32 = 13140;
    let mut cycle_count: u32 = 0;
    let mut reg_x: i32 = 1;
    let mut reg_x_snapshots: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("test1") {
        for l in lines.flatten() {
            parse_instr(l,  &mut cycle_count, &mut reg_x, &mut reg_x_snapshots);
        }
    }
    println!("res: {:?}", reg_x_snapshots );// p1: number of visited pos by the tail
    // let res = visited_pos.iter().len() as u32;
    println!("{}, {}", reg_x_snapshots[19], 20 * reg_x_snapshots[19]);
    println!("{}, {}", reg_x_snapshots[59], 60 * reg_x_snapshots[59]);
    println!("{}, {}", reg_x_snapshots[99], 100 * reg_x_snapshots[99]);
    println!("{}, {}", reg_x_snapshots[139], 140 * reg_x_snapshots[139]);
    println!("{}, {}", reg_x_snapshots[179], 180 * reg_x_snapshots[179]);
    println!("{}, {}", reg_x_snapshots[219], 220 * reg_x_snapshots[219]);
    let res: i32 = 20 * reg_x_snapshots[19] + 60 * reg_x_snapshots[59] + 100 * reg_x_snapshots[99] + 140* reg_x_snapshots[139] + 180* reg_x_snapshots[179] + 220* reg_x_snapshots[219];
    assert_eq!(expected, res);
}