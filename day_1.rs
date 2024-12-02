use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open the file
    let mut file = File::open("input.txt")?;

    // Read the file content into a string
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    // Each row in the text file contains two integers, separated by some spaces
    // We need to split the input into two vectors, one for each "column"
    let mut numbers_1: Vec<i32> = Vec::new();
    let mut numbers_2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();

        // Parse the integers and add them to the vectors
        numbers_1.push(numbers.next().unwrap().parse().unwrap());
        numbers_2.push(numbers.next().unwrap().parse().unwrap());
    }

    numbers_1.sort();
    numbers_2.sort();

    let mut sum = 0;
    for i in 0..numbers_1.len() {
        // Get the difference between this element in numbers_1 and numbers_2
        let diff = numbers_2[i] - numbers_1[i];

        // Add the difference to the sum
        sum += diff.abs();
    }

    println!("The sum of the differences is: {}", sum);

    // Part 2

    // For each element in numbers_1, how many times does it appear in numbers_2?
    let mut similarity_count = 0;
    for i in 0..numbers_1.len() {
        let mut count = 0;
        for j in 0..numbers_2.len() {
            if numbers_1[i] == numbers_2[j] {
                count += 1;
            }
        }
        let similarity = count * numbers_1[i];
        similarity_count += similarity;

    }
    
    println!("The similarity count is: {}", similarity_count);
    
    Ok(())
}
