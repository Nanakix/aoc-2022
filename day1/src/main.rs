use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = "../input.txt";
    
    let mut top_one_inventory: u32 = 0; // biggest inventory
    let mut top_two_inventory: u32 = 0;
    let mut top_three_inventory: u32 = 0;

    let mut inventory: u32 = 0;  // current inventory

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let tmp: &String = &line.unwrap();
            // counting '\n' chars as a number of elves' inventories
            if tmp.eq("") { 
                // When we update an elf's inventory, we must update the next ones
                if inventory > top_one_inventory {
                    top_three_inventory = top_two_inventory; 
                    top_two_inventory = top_one_inventory;
                    top_one_inventory = inventory;
                }
                else if inventory > top_two_inventory {
                    top_three_inventory = top_two_inventory;
                    top_two_inventory = inventory;
                }
                else if inventory > top_three_inventory {
                    top_three_inventory = inventory;
                }
                inventory = 0; // let's check another elf's backpack
            }
            else {
                let itmp: u32 = tmp.parse().unwrap();
                inventory = inventory + itmp; 
            }
        }
        println!("and the top three elves are carrying a total of {} + {} + {} = {} calories, lembas4life",
         top_one_inventory, top_two_inventory, top_three_inventory,
         top_one_inventory + top_two_inventory + top_three_inventory,);
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