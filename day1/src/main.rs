use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = "/home/nanakix/Documents/aoc-2022/day1/input.txt";
    println!("In file {}", file_path);
    let mut elf: u32 = 0; // counting '\n' chars as a number of elves
    
    let mut max_elf_one: u32 = 0; // the elf that carries the most calories
    let mut max_elf_two: u32 = 0;
    let mut max_elf_three: u32 = 0;
    
    let mut top_one_inventory: u32 = 0; // biggest inventory
    let mut top_two_inventory: u32 = 0;
    let mut top_three_inventory: u32 = 0;

    let mut inventory: u32 = 0;  // current inventory

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let tmp: &String = &line.unwrap();
            if tmp.eq("") {
                if inventory > top_one_inventory {
                    top_three_inventory = top_two_inventory;
                    max_elf_three = max_elf_two;
                    top_two_inventory = top_one_inventory;
                    max_elf_two = max_elf_one;
                    top_one_inventory = inventory;
                    max_elf_one = elf;
                }
                else if inventory > top_two_inventory {
                    top_three_inventory = top_two_inventory;
                    max_elf_three = max_elf_two;

                    top_two_inventory = inventory;
                    max_elf_two = elf;
                }
                else if inventory > top_three_inventory {
                    top_three_inventory = inventory;
                    max_elf_three = elf;
                }
                inventory = 0; // let's check another elf's backpack
                elf += 1;
            }
            else {
                let itmp: u32 = tmp.parse().unwrap();
                inventory = inventory + itmp; 
                // println!("adding {:#?} calories to the elf {} inventory", tmp, &elf);
                
            }
        }
        println!("and the best elf is {} carrying {} calories", elf, top_one_inventory);
        println!("and the top three elves are {}, {}, {}, carrying a total of {} + {} + {} = {} calories, lembas4life",
         max_elf_one, max_elf_two, max_elf_three,
         top_one_inventory, top_two_inventory, top_three_inventory,
         top_one_inventory + top_two_inventory + top_three_inventory,);
    }
}


fn update_elf(top_number: u32, inventory: u32){

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}