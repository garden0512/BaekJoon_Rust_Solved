use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let vector:Vec<i32> = buffer.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut remaining_cheese_frac_vector:Vec<i32> = Vec::new();
    remaining_cheese_frac_vector.push(vector[1] - vector[0]);
    remaining_cheese_frac_vector.push(vector[1]);
    let gcd_number = gcd(remaining_cheese_frac_vector[0], remaining_cheese_frac_vector[1]);
    remaining_cheese_frac_vector[0] /= gcd_number;
    remaining_cheese_frac_vector[1] /= gcd_number;
    println!("{} {}", remaining_cheese_frac_vector[0], remaining_cheese_frac_vector[1]);
}

fn gcd(mut a:i32, mut b:i32) -> i32
{
    let mut t:i32;
    while b != 0
    {
        t = b;
        b = a%b;
        a = t;
    }
    return a;
}