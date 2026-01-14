use std::io::Read;
use std::fs;

fn byte(filename:&str)-> std::io::Result<u64>{
    let mut file = std::fs::File::open(filename)?;
    let mut count = 0;
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        count += n as u64;
    }

    Ok(count)
    
}
fn words(filename: &str)->  usize{
    let mut wordcount = 0;
    for line in fs::read_to_string(filename).expect("REASON").lines(){
         for _word in line.split_whitespace() {
            wordcount += 1;
        }
    }
wordcount
}
fn newlines(filename: &str)-> usize{
    let mut linecount = 0;
    for line in fs::read_to_string(filename).expect("REASON").lines(){
         linecount += 1;
    }

 linecount
}


fn main() {
    let args: Vec<String> = std::env::args().collect();


    let filename = &args[1];

    let lines = newlines(filename);
    let words = words(filename);
    let bytes = byte(filename).expect("Failed to count bytes");

    println!("{} {} {} {}", lines, words, bytes, filename); 
}
