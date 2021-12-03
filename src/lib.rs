use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn binstr_to_dec(binstr: &str) -> i32 {
    i32::from_str_radix(binstr, 2).unwrap()
}
