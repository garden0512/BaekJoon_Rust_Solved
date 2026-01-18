use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let vector:Vec<char> = buffer.chars().map(|x| {
        match x
        {
            c if c.is_uppercase() => c.to_ascii_lowercase(),
            c if c.is_lowercase() => c.to_ascii_uppercase(),
            _ => x,
        }
    }).collect();
    buffer.clear();
    for character in vector
    {
        buffer.push(character);
    }
    println!("{}", buffer);
}