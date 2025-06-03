use std::{fs, io};

pub fn run1() -> Result<i64, io::Error> {
    let file_content = fs::read_to_string("input_exercice1.txt")?;

    let mut floor = 0;

    for c in file_content.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    Ok(floor)
}

pub fn run2() -> Result<i32, io::Error> {
    let file_content = fs::read_to_string("input_exercice1.txt")?;

    let mut floor = 0;
    let mut position = 0;

    for c in file_content.chars() {
        position+=1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            return Ok(position)
        }
    }

    Ok(floor)
}