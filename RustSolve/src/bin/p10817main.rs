use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    let mut vector:Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    vector.sort();
    println!("{}", vector[1]);
}