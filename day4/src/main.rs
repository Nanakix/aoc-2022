use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let file_path = "input";

    if let Ok(lines) = read_lines(file_path) {
        let mut contained: u32 = 0;
        for line in lines {
            match line {
                Ok(tmp) => {
                    // println!("{}", tmp);
                    let (a,b) = &tmp.split_once(",").unwrap_or_default();
                    let va = get_range(a);
                    let vb = get_range(b);
                        // part 2
                        if vb.iter().any(|item| va.contains(item)) {
                            contained += 1;
                        }
                        // end part 2
                        // part 1
                        // if vb.iter().all(|item| va.contains(item)) {
                        //     contained += 1;
                        // }
                        // else if va.iter().all(|item| vb.contains(item)){
                        //     contained += 1;     
                        // }
                        // end part 1
                        else {
                            println!("nope");
                        }
                },
                Err(_e) => println!("Error reading line")
            }
        }
        println!("total contained ranges: {}", contained); // result
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

fn get_range (elf: &str) -> Vec<u32>{
    let (one, two) = elf.split_once("-").unwrap_or_default();
    let begin: u32 = one.parse::<>().unwrap();
    let end: u32 = two.parse::<>().unwrap();
    let v = (begin..end+1).collect::<Vec<_>>(); // why +1 ? I dunno
    println!("{} {} {:?}", &begin, &end, &v);
    v
    //FIXME: reduce complexity by just checking min and max limits instead of adding n elts (exploding complexity)
}