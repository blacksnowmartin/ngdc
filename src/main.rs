use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    let content = read_file(filename)?;
    
    // Functionality 1: Count lines
    let line_count = content.lines().count();
    println!("Number of lines: {}", line_count);

    // Functionality 2: Count words
    let word_count = content.split_whitespace().count();
    println!("Number of words: {}", word_count);

    // Functionality 3: Count characters
    let char_count = content.chars().count();
    println!("Number of characters: {}", char_count);

    // Functionality 4: Most common word
    let most_common_word = most_common_word(&content);
    println!("Most common word: {:?}", most_common_word);

    // Functionality 5: Reverse content
    let reversed_content = content.chars().rev().collect::<String>();
    write_file("reversed.txt", &reversed_content)?;

    // Functionality 6: Convert to uppercase
    let uppercase_content = content.to_uppercase();
    write_file("uppercase.txt", &uppercase_content)?;

    // Functionality 7: Convert to lowercase
    let lowercase_content = content.to_lowercase();
    write_file("lowercase.txt", &lowercase_content)?;

    // Functionality 8: Replace word
    if args.len() == 4 {
        let old_word = &args[2];
        let new_word = &args[3];
        let replaced_content = content.replace(old_word, new_word);
        write_file("replaced.txt", &replaced_content)?;
    } else {
        eprintln!("Usage: {} <filename> <old_word> <new_word>", args[0]);
        std::process::exit(1);
    }

    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut content = String::new();
    io::BufReader::new(file).read_to_string(&mut content)?;
    Ok(content)
}

fn write_file(filename: &str, content: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn most_common_word(content: &str) -> Option<String> {
    let mut word_counts = HashMap::new();
    for word in content.split_whitespace() {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    word_counts.into_iter().max_by_key(|&(_, count)| count).map(|(word, _)| word)
}
