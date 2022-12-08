use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = "input";
    let mut v : Vec<VecDeque<u8>> = Vec::new();
    v.resize(9, VecDeque::with_capacity(8));
    if let Ok(lines) = read_lines(file_path) {
        for (index, l) in lines.flat_map(|l| l).enumerate() {
            // construct all stacks
            if index < 9 { // stacks are written from line 1 to 9 in input, too lazy to fetch it generically
                for (i, chunk) in l.as_bytes().chunks(4).enumerate() {
                    if chunk[1] != 32 {
                        v[i].push_front(chunk[1]);
                    }
                }
            }
            else {
                // parse the moves
                let load: Vec<u8> = l.split(" ").skip(1).step_by(2).flat_map(|w| w.parse::<u8>()).collect();
                // do the moves
                //p1
                // println!("{:?}", &v);
                // for _ in 0..load[0] {
                //     let tmp = v[(load[1]-1) as usize].pop_back().unwrap_or_default();
                //     v[(load[2]-1) as usize].push_back(tmp);
                // }
                //p2
                let mut holder: Vec<u8> = Vec::new();//9001 exclusive feature
                for _ in 0..load[0] {
                    let tmp = v[(load[1]-1) as usize].pop_back().unwrap_or_default();
                    holder.push(tmp);
                }
                for _ in 0..holder.len() {
                    let tmp = holder.pop().unwrap();
                    v[(load[2]-1) as usize].push_back(tmp);
                }
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