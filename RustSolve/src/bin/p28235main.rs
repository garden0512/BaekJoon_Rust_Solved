use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    match buffer.trim(){
        "SONGDO" => println!("HIGHSCHOOL"),
        "CODE" => println!("MASTER"),
        "2023" => println!("0611"),
        "ALGORITHM" => println!("CONTEST"),
        _ => panic!("오류"),
    }
}