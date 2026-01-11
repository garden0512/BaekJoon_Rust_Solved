use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    let vector:Vec<i32> = buffer
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let area:f64 = (vector[0] as f64 * vector[1] as f64) / 2.0;
    println!("{:.1}", area);
}