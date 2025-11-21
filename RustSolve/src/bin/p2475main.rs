use std::io::{stdin};

fn main()
{
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .unwrap();
    let v:Vec<i32> = input
        .split_whitespace()
        .map(|input| input.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let square_vector:Vec<i32> = v
        .into_iter()
        .map(|x| x*x)
        .collect::<Vec<i32>>();
    let summ:i32 = square_vector
        .iter()
        .sum();
    println!("{}", summ % 10);
}