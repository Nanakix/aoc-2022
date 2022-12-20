use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let file_path = "input";
    let mut forest: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for l in lines.flatten() {
            forest.push(l.chars().collect());
        }
    }
    println!("res: {}", count_visibles(&forest));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn count_visibles(forest: &Vec<Vec<char>>) -> u32 {
    let mut cpt: u32 = 0;
    for (y, r) in forest.into_iter().enumerate() {
        for (x, tree) in r.into_iter().enumerate() {
            if y == 0 || x == 0 || x == r.len() - 1 || y == forest.len() - 1 { // if the tree is on the edge of the forest
                cpt += 1;
            }
            else {
                if check_visible(forest, y, x, tree) {
                     cpt += 1;
                }
            }
        }
    }
    cpt
}

fn check_visible(forest: &Vec<Vec<char>>,  x: usize, y: usize, tree: &char) -> bool {
    let mut side: u32 = 4;
    let width = forest[0].len();
    // check each side, if each side has a tree as tall or taller than @param: tree, then it is not visible
    if forest[0..x].iter().map(|row| row[y]).any(|e| e >= *tree) {
        side -= 1; 
    }
    if forest[x+1..forest.len()].iter().map(|row| row[y]).any(|e| e >= *tree) {
        side -= 1;
    }
    if forest[x][0..y].iter().any(|e| e >= tree) {
        side -= 1;
    }
    if forest[x][y+1..width].iter().any(|e| e >= tree) {
        side -= 1;
    }
    side != 0
}

#[test]
fn test_parse() {
    let expected: u32 = 210;
    let mut forest: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("test1") {
        for l in lines.flatten() {
            forest.push(l.chars().collect());
        }
    }
    let res: u32 = count_visibles(&forest);
    assert_eq!(expected, res);
}
