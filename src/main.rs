use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use regex::Regex;
use std::fmt;

const DEBUG: bool = false;
fn debug(statement: &str) {
    if DEBUG {
        println!("{}", statement);
    }
}

fn main() {
    let output_file = "output.json";
    let directory = "test";
    let mut output = Output::default(directory.to_string());
    let (_unsafe_lines, _all_lines) = match traverse_dir(&mut output, directory) {
        Ok((unsafe_lines, all_lines)) => (unsafe_lines, all_lines),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("{}", output.borrow());
    // write to a json file
    match output.borrow().export_to_file(output_file) {
        Ok(_) => println!("Output written to output.json"),
        Err(e) => println!("Error writing to output.json: {}", e),
    };
}

// output tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    path: String,
    unsafe_lines: u64,
    all_lines: u64,
    children: Vec<OutputRef>,
}

type OutputRef = Rc<RefCell<Output>>; // give reference an alias
impl Output {
    pub fn default(path: String) -> OutputRef {
        Rc::new(RefCell::new(Output {
            path,
            unsafe_lines: 0,
            all_lines: 0,
            children: Vec::new(),
        }))
    }
    pub fn new(path: String, unsafe_lines: u64, all_lines: u64) -> OutputRef {
        Rc::new(RefCell::new(Output {
            path,
            unsafe_lines,
            all_lines,
            children: Vec::new(),
        }))
    }
    pub fn set_unsafe_lines(&mut self, unsafe_lines: u64) {
        self.unsafe_lines = unsafe_lines;
    }
    pub fn set_all_lines(&mut self, all_lines: u64) {
        self.all_lines = all_lines;
    }
    pub fn add_child(&mut self, child: OutputRef) {
        self.children.push(child);
    }
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
    pub fn export_to_file(&self, filename: &str) -> Result<()> {
        let json = self.to_json()?;
        std::fs::write(filename, json)?;
        Ok(())
    }
}
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = format!(
      "\n------------------------------------\n{}\n------------------------------------\n{} unsafe lines\n{} total lines\nunsafe:safe ratio: {}%\n\n",
      self.path,
      self.unsafe_lines,
      self.all_lines,
      self.unsafe_lines as f64 / self.all_lines as f64
    );
        for child in &self.children {
            text.push_str(&format!("\t{}", child.borrow()));
        }
        write!(f, "{}", text)
    }
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

// walk through a directory and its subdirectories, and call detect_unsafe on each rust file
pub fn traverse_dir(output: &mut OutputRef, dir: &str) -> Result<(u64, u64)> {
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
            // add child output tree
            let child = Output::new(path_str.to_string(), usf_lines, lines);
            output.borrow_mut().add_child(child);
        }
        // check if path is a directory
        else if full_path.is_dir() {
            let mut nested_output = Output::default(path_str.to_string());
            let (usf_lines, lines) = traverse_dir(&mut nested_output, path_str)?;
            unsafe_lines += usf_lines;
            all_lines += lines;
            // set child vals
            nested_output.borrow_mut().set_unsafe_lines(usf_lines);
            nested_output.borrow_mut().set_all_lines(lines);
            // add child output tree
            output.borrow_mut().add_child(nested_output);
        }
    }
    output.borrow_mut().set_unsafe_lines(unsafe_lines);
    output.borrow_mut().set_all_lines(all_lines);
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
        let mut output: Rc<RefCell<Output>> = Output::default("test/test_dir".to_string());
        let (unsafe_lines, all_lines) = traverse_dir(&mut output, "test/test_dir").unwrap();
        assert_eq!(unsafe_lines, 6);
        assert_eq!(all_lines, 28);
    }

    #[test]
    fn test_recursive_traverse_dir() {
        let mut output: Rc<RefCell<Output>> = Output::default("test/test_dir".to_string());
        let (unsafe_lines, all_lines) =
            traverse_dir(&mut output, "test/recursive_test_dir").unwrap();
        assert_eq!(unsafe_lines, 6);
        assert_eq!(all_lines, 28);
    }
}
