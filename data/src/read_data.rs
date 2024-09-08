use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_sample_data(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new("samples").join(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    
    reader.lines().collect()
}

pub fn main() -> io::Result<()> {
    let lines = read_sample_data("one_day_sample.txt")?;
    
    for line in lines {
        println!("{}", line);
    }
    
    Ok(())
}