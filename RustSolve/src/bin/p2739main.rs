use std::io::{stdin};

fn main()
{
    let mut input:String = String::new();
    stdin()
        .read_line(&mut input)
        .unwrap();
    let v:Vec<i32> = input
        .split_whitespace()
        .map(|input| input.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for i in 1..10
    {
        println!("{} * {} = {}", v[0], i, v[0]*i);
    }
}