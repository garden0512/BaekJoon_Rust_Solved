use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    let mut vector:Vec<i32> = Vec::new();
    for _ in 0..2
    {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let n = buffer.trim().parse::<i32>().unwrap();
        vector.push(n);
    }
    println!("{}" , vector[0] * vector[1]);
}