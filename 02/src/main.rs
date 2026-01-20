use std::io::Read;
use std::fs;
struct Flags{
    show_lines:bool,
    show_words: bool,
    show_bytes: bool,
    show_length: bool,
    show_chars: bool,
}

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
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .split_whitespace()
        .count()
}

fn newlines(filename: &str) -> usize {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .count()
}
fn chars(filename: &str) -> usize{
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .chars()
        .count()
}
fn line_length(filename: &str) -> usize {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.len())
        .max()
        .unwrap_or(0)
}

fn charflg(arg: &str, flags: &mut Flags){
    match arg{
        "c" => {
            flags.show_bytes = true
        }
        "l" =>{
           flags.show_lines = true    
        }
        "m" =>{
            flags.show_chars = true
        }
        "w" =>{
            flags.show_words = true
        }
        "L" =>{
            flags.show_length = true
        }
        _=>{}
    }
}
fn strflg(arg: &str, flags: &mut Flags){
    match arg{
        "bytes" => {
            flags.show_bytes = true
        }
        "lines" =>{
           flags.show_lines = true
        }
        "chars"=>{
            flags.show_chars = true
        }
        "words" =>{
            flags.show_words = true
        }
        "max-line-length" =>{
            flags.show_length = true
        }
        "help" => {
            println!("Usage: my_wc [OPTION]... [FILE]...");
            println!("Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.");
            println!("A word is a non-zero-length sequence of characters delimited by white space.\n");
            println!("With no FILE, or when FILE is -, read standard input.\n");
            println!("Options (always printed in order: newline, word, character, byte, maximum line length):");
            println!("  -c, --bytes            print the byte counts");
            println!("  -m, --chars            print the character counts (not implemented)");
            println!("  -l, --lines            print the newline counts");
            println!("  -L, --max-line-length  print the maximum line length");
            println!("  -w, --words            print the word counts");
            println!("      --help             display this help and exit");
            println!("      --version          output version information and exit");
            std::process::exit(0);
        }
        "version" => {
            println!("my_wc (Caden Dirnberger) 2.0");
            println!("Copyright (C) 2026 Caden Dirnberger");
            println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>");
            println!("This is free software: you are free to change and redistribute it.");
            println!("There is NO WARRANTY, to the extent permitted by law.");
            std::process::exit(0);
        }
        _ => {
            eprintln!("Invalid flag: --{}", arg);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut filename: Option<String> = None;
    let mut flags = Flags {
        show_lines: false,
        show_words: false,
        show_bytes: false,
        show_length: false,
        show_chars: false,
    };
    for arg in &args[1..]{
        match arg.as_str(){
            s if s.starts_with("--")=>{
                strflg(s.trim_start_matches("--"), &mut flags);
                
            }
            s if s.starts_with("-")=>{
                for ch in s.trim_start_matches("-").chars() {
                    charflg(&ch.to_string(), &mut flags);
                }
            }
            _ =>{
                filename = Some(arg.to_string());
            }
        }
    }
    let filename = match filename {
        Some(f) => f,
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");
            let temp_file = "/tmp/my_wc_temp.txt";
            std::fs::write(temp_file, &buffer).expect("Failed to write temp file");
            temp_file.to_string()
        }
    };
    if !flags.show_lines && !flags.show_words && !flags.show_bytes && !flags.show_length && !flags.show_chars {
        flags.show_lines = true;
        flags.show_words = true;
        flags.show_bytes = true;
    }
    let lines = newlines(&filename);
    let words = words(&filename);
    let chars = chars(&filename);
    let bytes = byte(&filename).expect("Failed to count bytes");
    let length = line_length(&filename);
    if flags.show_lines{
    print!("{} ", lines)
    }
    if flags.show_words{
    print!("{} ", words)
    }
    if flags.show_bytes{
    print!("{} ", bytes)
    }
    if flags.show_length {
    print!("{} ", length)
    }
    if flags.show_chars{
    print!("{} ", chars)
    }
    if args.len() > 1 || filename != "/tmp/my_wc_temp.txt" {
        println!("{}", filename);
    } else {
        println!("");
    }
    
}
