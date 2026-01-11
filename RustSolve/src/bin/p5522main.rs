use std::io::{stdin};

fn main()
{
    let mut sum:i32 = 0;
    let mut buffer = String::new();
    for _ in 0..5
    {
        loop
        {
            buffer.clear();
            stdin()
                .read_line(&mut buffer)
                .unwrap();
            match buffer
                .trim()
                .parse::<i32>()
            {
                Ok(num) if(0..=100).contains(&num) =>
                    {
                        sum += num;
                        break;
                    }
                _ =>
                    {
                        panic!("에러")
                    }
            }
        }
    }
    println!("{}", sum);
}