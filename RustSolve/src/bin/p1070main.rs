use std::io::{stdin};

fn main()
{
    let mut buffer = String::new();
    let mut vector:Vec<i32> = Vec::new();
    for _ in 0..5 {
        buffer.clear();
        stdin()
            .read_line(&mut buffer)
            .unwrap();
        vector.push(buffer.trim().parse::<i32>().unwrap());
    }
    let x_used = vector[4] * vector[0];
    let y_used = if vector[4] <= vector[2]
        { 
            vector[1] 
        }
        else 
        { 
            vector[1] + vector[3] * (vector[4] - vector[2]) 
        };
    println!("{}" , if x_used <= y_used{
        x_used
    }
    else
    {
        y_used
    });
}