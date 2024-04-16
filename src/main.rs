use anyhow::{anyhow, Result};
use regex::Regex;

const DEBUG: bool = false;
fn debug(statement: &str) {
    if DEBUG {
        println!("{}", statement);
    }
}

fn main() {
    let (unsafe_lines, all_lines) = match traverse_dir("parse_dir") {
        Ok((unsafe_lines, all_lines)) => (unsafe_lines, all_lines),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!(
        "unsafe ratio {}/{}, or {}",
        unsafe_lines,
        all_lines,
        unsafe_lines as f64 / all_lines as f64
    );
}

fn contains_unsafe(line: &str) -> Result<bool> {
    let re = Regex::new(r"\sunsafe\s\{")?;
    Ok(re.is_match(line))
}

// returns the number of lines in an unsafe block, and the total number of lines
pub fn detect_unsafe(filename: &str) -> Result<(u64, u64)> {
    // check if file ends in .rs
    if !filename.ends_with(".rs") {
        return Err(anyhow!("File must be a Rust file"));
    }
    // check if file exists
    if !std::path::Path::new(&filename).exists() {
        return Err(anyhow!("File does not exist"));
    }
    // read file into string
    let file = match std::fs::read_to_string(filename) {
        Ok(file) => file,
        Err(_) => return Err(anyhow!("Could not read file")),
    };
    // check if file contains "unsafe"
    let mut line_number = 0;
    let mut unsafe_lines = 0;
    let mut in_unsafe_block = false;
    let mut unsafe_vec = Vec::<String>::new(); // unsafe vec will be a back-stack, popping and pushing from the back
    for line in file.lines() {
        line_number += 1;
        if contains_unsafe(line)? || in_unsafe_block {
            debug(&format!("{}: {}", line_number, line));
            in_unsafe_block = true;
            unsafe_lines += 1;
            // push every { and } to a vector
            for c in line.chars() {
                if c == '{' {
                    unsafe_vec.push(c.to_string());
                } else if c == '}' {
                    unsafe_vec.pop();
                }
            }
            // if the vector is empty, we are out of the unsafe block
            if unsafe_vec.is_empty() {
                in_unsafe_block = false;
            }
        }
    }

    return Ok((unsafe_lines, line_number));
}

pub fn traverse_dir(dir: &str) -> Result<(u64, u64)> {
    let mut all_lines = 0;
    let mut unsafe_lines = 0;
    let paths = std::fs::read_dir(dir)?;
    for path in paths {
        let path = path?;
        let full_path = path.path();
        let path_str = full_path.to_str().unwrap();
        // check if path is a rust file
        if path_str.ends_with(".rs") {
            let (usf_lines, lines) = detect_unsafe(path_str)?;
            unsafe_lines += usf_lines;
            all_lines += lines;
        }
        // check if path is a directory
        else if full_path.is_dir() {
            let (usf_lines, lines) = traverse_dir(path_str)?;
            unsafe_lines += usf_lines;
            all_lines += lines;
        }
    }
    Ok((unsafe_lines, all_lines))
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_unsafe() {
        let (unsafe_lines, all_lines) = detect_unsafe("test/test.rs").unwrap();
        assert_eq!(unsafe_lines, 3);
        assert_eq!(all_lines, 14);
    }

    #[test]
    fn test_traverse_dir() {
        let (unsafe_lines, all_lines) = traverse_dir("test/test_dir").unwrap();
        assert_eq!(unsafe_lines, 6);
        assert_eq!(all_lines, 28);
    }

    #[test]
    fn test_recursive_traverse_dir() {
        let (unsafe_lines, all_lines) = traverse_dir("test/recursive_test_dir").unwrap();
        assert_eq!(unsafe_lines, 6);
        assert_eq!(all_lines, 28);
    }
}