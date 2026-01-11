use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    let number_of_repetitions = buffer.trim().parse::<i32>().unwrap();
    for _ in 0..number_of_repetitions{
        buffer.clear();
        stdin()
            .read_line(&mut buffer)
            .unwrap();
        let vector:Vec<i32> = buffer
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        println!("{}", vector[0] * (vector[2] - 1) + vector[1]);
    }
}