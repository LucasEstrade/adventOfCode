use std::collections::HashMap;

fn main() {

    // Part 1 - Calculate the sum of the absolute differences between the right and left columns of the input file
    /* 
    let input = include_str!("input.txt");

    let lines_vec: Vec<&str> = input.lines().collect();
    let mut right_column : Vec<i32> = vec![];
    let mut left_column : Vec<i32> = vec![];

    for line in lines_vec.iter() {
       right_column.push(line.split("   ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap());
       left_column.push(line.split("   ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap());
    }

    right_column.sort();
    left_column.sort();

    let mut total_sum = 0;

    for i in 0..right_column.len() {
        total_sum += (right_column[i] - left_column[i]).abs();
    }

    println!("Total sum part one: {}", total_sum);

    */

    // Part 2 - Calculate the similarity score between the right and left columns of the input file
    let mut similarity_score = 0;

    let input = include_str!("input.txt");

    let lines_vec: Vec<&str> = input.lines().collect();
    let mut right_column : HashMap<i32,i32> =  HashMap::new();
    let mut left_column : Vec<i32> = vec![];

    for line in lines_vec.iter() {
       left_column.push(line.split("   ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap());
       right_column.entry(line.split("   ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()).and_modify(|e| *e += 1).or_insert(1);
    }

    for i in left_column.iter() {
        if right_column.contains_key(&i) {
            similarity_score += i * right_column.get(&i).unwrap();
        }
    }

    println!("Similarity score: {}", similarity_score);

}
