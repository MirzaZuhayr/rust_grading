use std::io;

fn main() {
    println!("Please input your grade"); 

    let mut grade = u8::new();

    io::stdin()
        .read_line(&mut grade)
        .expect("Failed to read line");
    if grade > 90 {
        println!("A");
    } else { 
        println!("F");
    }

    
}
