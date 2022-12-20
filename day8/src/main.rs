use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let file_path = "input";
    let mut forest: Vec<Vec<char>> = Vec::new();
    // let mut visible_trees: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for l in lines.flatten() {
            forest.push(l.chars().collect());
        }
    }
    println!("res: {}", count_visibles(&forest, /*&mut visible_trees*/));
    // for vl in visible_trees {
        // for vt in vl {
            // print!("{}", vt);
        // }
        // println("{:?}", vl);
        // println!();
    // }
    // println!("res: {}", res);// p1: sum of the dirs of size <= 100000
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn count_visibles(forest: &Vec<Vec<char>>,  /*visible_trees: &mut Vec<Vec<char>>*/) -> u32 {
    let mut cpt: u32 = 0;
    for (y, r) in forest.into_iter().enumerate() {

        for (x, tree) in r.into_iter().enumerate() {
            if y == 0 || x == 0 || x == r.len() - 1 || y == forest.len() - 1 { // if the tree is on the edge of the forest
                cpt += 1;
                // visible_trees[y].push(tree.clone());
                // println!("{}", tree);
            }
            else {
                // if check_row(forest, x, y, tree) { cpt += 1;}
                // visible_trees[y].push(' ');
                if check_column(forest, y, x, tree) { cpt += 1;}
            }
        }
    }
    cpt
}

fn check_column(forest: &Vec<Vec<char>>,  x: usize, y: usize, tree: &char) -> bool {
    // let mut res: bool = true;
    let mut side: u32 = 4;
    let width = forest[0].len();
    // if forest[0..x+1][y].iter().any(|e| e > tree) {
    if forest[0..x].iter().map(|row| row[y]).any(|e| e >= *tree) {
        // println!("e: {}, tree: {}", e, tree);
        println!("rip1 petit arbre {}, tu n'es pas le plus grand de ta ligne en partant vers la gauche", tree);
        side -= 1; 
    }
    // if forest[x+1..forest.len()][y].iter().any(|e| e > tree) {
    if forest[x+1..forest.len()].iter().map(|row| row[y]).any(|e| e >= *tree) {
        println!("rip2 petit arbre {}, tu n'es pas le plus grand de ta ligne en partant vers la droite", tree);
        side -= 1;
    }
    if forest[x][0..y].iter().any(|e| e >= tree) {
        println!("rip3 petit arbre {}, tu n'es pas le plus grand de ta colonne en partant vers le haut", tree);
        side -= 1;
    }
    if forest[x][y+1..width].iter().any(|e| e >= tree) {
        println!("rip4 petit arbre {}, tu n'es pas le plus grand de ta ligne en partant vers le bas", tree);
        side -= 1;
    }
    side != 0
}

// fn check_row(forest: &Vec<Vec<char>>,  x: usize, y: usize, tree: &char) -> bool {
//     let mut res: bool = true;
//     if forest[0..y+1].iter().any(|e| e >= *tree) {
//         println!("rip3 petit arbre {}, tu n'es pas le plus grand de ta colonne", tree);
//         res = false; 
//     }
//     if forest[y+1..forest.len()].iter().map(|row| row[x]).any(|e| e >= *tree) {
//         println!("rip4 petit arbre {}, tu n'es pas le plus grand de ta colonne", tree);
//         res = false; 
//     }
//     res
// }

#[test]
fn test_parse() {
    let expected: u32 = 210;
    let mut forest: Vec<Vec<char>> = Vec::new();
    // let mut visible_trees: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("test1") {
        for l in lines.flatten() {
            forest.push(l.chars().collect());
        }
    }
    let res: u32 = count_visibles(&forest, /*&mut visible_trees*/);
    // println!("res: {}", res);
    assert_eq!(expected, res);
    // for vl in visible_trees {
        // for vt in vl {
            // print!("{}", vt);
        // }
        // println("{:?}", vl);
        // j'essaie de représenter visuellement les arbres visibles via visible_trees
        // println!();
    // }
}

// #[test]
// fn test() {
//     let a = [1,2,3,4,5];
//     println!("{:?}", a[0..5]);
// }