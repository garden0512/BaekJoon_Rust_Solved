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
    match (vector[0] , vector[1]){
        (12..=16, 0) if vector[1] == 0 =>{
            println!("320");
        }
        _ => {
            println!("280");
        }
    }
}