use std::collections::{HashMap};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


const P1_SIZE: u32 = 100000;
fn main() {
    let file_path = "input2";
    let mut current_path_stack: Vec<String> = Vec::new();
    let mut rep_sizes: HashMap<String, u32> = HashMap::new();
    if let Ok(lines) = read_lines(file_path) {
        for l in lines.flatten() {
            parse_hier(l, &mut rep_sizes, &mut current_path_stack); // p1
        }
    }
    rep_sizes.retain(|_, v| *v <= P1_SIZE ); // return only dirs with size < P1_SIZE
    println!("rep_sizes: {:?}", rep_sizes);
    let res = rep_sizes.values().sum::<u32>();  
    println!("res: {}", res);// p1: sum of the dirs of size <= 100000
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}


fn parse_hier(s: String, dirs: &mut HashMap<String, u32>, current_path_stack: &mut Vec<String>) {
    let mut sp: Vec<&str> = s.split(" ").collect();
    match sp[0] {
        "$" => {
            parse_cmd(sp.drain(1..).collect(),  dirs, current_path_stack); 
        },
        "dir" => { 
            // dirs.entry(sp[1].to_string()).or_insert(0);
        },
        _ => {
            let nb: u32 = sp[0].get(..sp[0].len()).unwrap().parse().unwrap(); // if it's a number
            // println!("current stack: {:?}", current_path_stack);
            for i in 0..current_path_stack.len()+1 {
                dirs.entry(current_path_stack[0..i].concat()).and_modify(|v| *v += nb);
            }
        }
    }    
}

fn parse_cmd(cmd: Vec<&str>,  dirs: &mut HashMap<String, u32>, current_path_stack: &mut Vec<String>) {
    match cmd[0] {
        "cd" => {
            match cmd[1] {
                "/" => { *current_path_stack = vec!["/".to_string()] },
                ".." => { current_path_stack.pop().unwrap_or_default(); },
                _ => {
                    current_path_stack.push(cmd[1].to_string());
                    dirs.entry(current_path_stack.concat()).or_insert(0);
                },
            };
        },
        "ls" => (),
        _ => panic!("{} RIP", cmd[0]),
    }
}

#[test]
fn test_parse() {
    let expected: u32 = 95437;
    let mut current_path_stack: Vec<String> = Vec::new();
    let mut rep_sizes: HashMap<String, u32> = HashMap::new();
    if let Ok(lines) = read_lines("test1") {
        for l in lines.flatten() {
            parse_hier(l, &mut rep_sizes, &mut current_path_stack);
        }
    }
    rep_sizes.retain(|_, v| *v <= P1_SIZE ); // return only dirs with size < P1_SIZE
    println!("{:?}", rep_sizes);
    let res = rep_sizes.values().sum::<u32>();
    println!("{}", res);
    assert_eq!(expected, res);
}
