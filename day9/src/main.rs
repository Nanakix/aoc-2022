use std::collections::{HashSet};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;


fn main() {
    let file_path = "input";
    let mut visited_pos: HashSet<(i32,i32)> = HashSet::new();
    let mut knots: Vec<(i32, i32)> = vec![(0,0);10];
    // let mut directions: HashSet<String, u32> = HashSet::new();
    
    if let Ok(lines) = read_lines(file_path) {
        for l in lines.flatten() {
            parse_ropes(l, &mut knots, &mut visited_pos);
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

fn check_positions(knots: &mut Vec<(i32, i32)>, visited_pos: &mut HashSet<(i32,i32)>) {


    for i in 0..knots.len() - 1 {
        // if knots[i+1].0 
        if (knots[i].0 - knots[i+1].0) .abs() > 1 || (knots[i].1 - knots[i+1].1) .abs() > 1 {
            knots[i + 1].0 += (knots[i].0 - knots[i+1].0).signum(); 
            knots[i + 1].1 += (knots[i].1 - knots[i+1].1).signum(); 
        }
        
    }
    visited_pos.insert(knots[knots.len() -1]);
    println!("{:?}", knots[knots.len() -1]);


    // for (i, _) in knots.iter().enumerate() {
    //     if i < knots.len() - 1 {

    //         if ((knots[i].0- knots[i+1].0).abs() > 1 && (knots[i].1 - knots[i+1].1).abs() > 0) || ((knots[i].0- knots[i+1].0).abs() > 0 && (knots[i].1 - knots[i+1].1).abs() > 1) {
    //             // tester le sens enfin
    //             if knots[i].0 > knots[i+1].0 { //RIGHT
    //                 knots[i+1].0 += 1;
    //             }
    //             else { // LEFT
    //                 knots[i+1].0 -= 1;
    //             }
    //             if knots[i].1 > knots[i+1].1 { // UP
    //                 knots[i+1].1 += 1;
    //             }
    //             else { // DOWN
    //                     knots[i+1].1 -= 1;
    //                 }
    //             visited_pos.insert(knots[i+1].clone());
    //         }
    //         if (knots[i].0 - knots[i+1].0).abs() > 1 {
    //             if knots[i].0 > knots[i+1].0 { //RIGHT
    //                 knots[i+1].0 += 1;
    //             }
    //             else { // LEFT
    //                 knots[i+1].0 -= 1;
    //             }
    //             println!("MOVE knots[i]: {:?} knots[i+1]: {:?}", &knots[i], &knots[i+1]);  
    //         }
    //         visited_pos.insert(knots[i+1].clone());
    //         if (knots[i].1 - knots[i+1].1).abs() > 1 {
    //             if knots[i].1 > knots[i+1].1 { // UP
    //                 knots[i+1].1 += 1;
    //             }
    //             else { // DOWN
    //                 knots[i+1].1 -= 1;
    //             }
    //             println!("MOVE knots[i]: {:?} knots[i+1]: {:?}", &knots[i], &knots[i+1]);  
    //         }    
    //         if i == knots.len() - 1{
    //             visited_pos.insert(knots[i+1].clone());            
    //         }
    //     }

    // } // end for

}

fn parse_ropes(s: String, knots: &mut Vec<(i32, i32)>, visited_pos: &mut HashSet<(i32,i32)>) {
    let sp: Vec<&str> = s.split(" ").collect();
    let nb: u32 = sp[1].parse().unwrap(); // if it's a number
    // up down Y, right left X
    visited_pos.insert((0,0));
    for _ in 0..nb {
        match sp[0] { // direction
            "U" => {
                    knots[0].1 += 1;
                    println!("Up: HEAD: {:?}", &knots.first().unwrap());  
            },
            "D" => { 
                    knots[0].1 -= 1;
                    println!("Down: HEAD: {:?}", &knots[0]);  
            },
            "L" => {
                knots[0].0 -= 1;
                println!("Left: HEAD: {:?}", &knots[0]);  
            },
            "R" => { 
                knots[0].0 += 1;
                println!("Right: HEAD: {:?}", &knots[0]);  
            },
            _ => {
                panic!("wtf");
            }
        }
        check_positions(knots, visited_pos);
    }
}   

#[test]
fn test_parse1() {
    let expected: u32 = 13;
    let mut knots: Vec<(i32, i32)> = vec![(0,0),(0,0)];
    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();
    if let Ok(lines) = read_lines("test") {
        for l in lines.flatten() {
            parse_ropes(l, &mut knots, &mut visited_pos)
        }
    }
    println!("res: {}", visited_pos.iter().len() );// p1: number of visited pos by the tail
    
    let res = visited_pos.iter().len() as u32;
    assert_eq!(expected, res);
}

#[test]
fn test_parse() {
    let expected: u32 = 36;
    let mut knots: Vec<(i32, i32)> = vec![(0,0);10];
    let mut visited_pos: HashSet<(i32, i32)> = HashSet::new();
    if let Ok(lines) = read_lines("test2") {
        for l in lines.flatten() {
            parse_ropes(l,  &mut knots, &mut visited_pos);
            // rendre générique parse_ropes en parcourant pour chaque noeud de la corde, genre 
            // on lui envoie noeud et noeud.next et ça fait tout les mouvements pour chaque noeud de la corde. 
            // parse_ropes(l, &mut head, &mut tail, &mut visited_pos)
        }
    }
    println!("res: {}", visited_pos.iter().len() );// p1: number of visited pos by the tail
    let res = visited_pos.iter().len() as u32;
    assert_eq!(expected, res);
}