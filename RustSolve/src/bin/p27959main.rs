use std::io::{stdin};

fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v:Vec<i32> = input.split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect();
    if v[0] * 100 >= v[1]
    {
        println!("Yes");
    }
    else
    {
        println!("No");
    }
}