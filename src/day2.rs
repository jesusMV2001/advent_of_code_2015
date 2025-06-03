use std::{fs, io};

pub fn run1() -> Result<i32, io::Error> {
    let input_content = fs::read_to_string("input_exercice2.txt")?;

    let mut total = 0;
    
    for line in input_content.lines(){
        let mut measures: Vec<i32> = line.split('x')
            .map(|s| s.parse().unwrap())
            .collect();

        let l = measures[0];
        let w = measures[1];
        let h = measures[2];
        let min = min_area(&mut measures);
        let dimension = 2*l*w + 2*w*h + 2*h*l + min;
        
        total += dimension;
    }

    Ok(total)
}

pub fn run2() -> Result<i32, io::Error> {
    let input_content = fs::read_to_string("input_exercice2.txt")?;

    let mut total = 0;

    for line in input_content.lines(){
        let mut measures: Vec<i32> = line.split('x')
            .map(|s| s.parse().unwrap())
            .collect();

        let l = measures[0];
        let w = measures[1];
        let h = measures[2];
        let min = min_perimeter(&mut measures);
        let dimension = l*w*h + min;

        total += dimension;
    }

    Ok(total)
}

fn min_perimeter(measures: &mut Vec<i32>) -> i32 {
    measures.sort();
    
    measures[0]*2 + measures[1]*2
}

fn min_area(measures: &mut Vec<i32>) -> i32 {
    measures.sort();
    
    measures[0] * measures[1]
}