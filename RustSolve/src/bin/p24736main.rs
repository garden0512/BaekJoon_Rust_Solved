use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    let mut score_count_vector:Vec<i32> = Vec::new();
    for i in 0..2
    {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

    }
    let score_count_vector:Vec<i32> = buffer.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
}