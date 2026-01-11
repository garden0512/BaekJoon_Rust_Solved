use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .unwrap();
    match buffer.trim() {
        "M" => println!("MatKor"),
        "W" => println!("WiCys"),
        "C" => println!("CyKor"),
        "A" => println!("AlKor"),
        "$" => println!("$clear"),
        _ => panic!("오류")
    }
}