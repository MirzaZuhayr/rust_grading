use std::io;

fn main() {
    println!("Please input your grade"); 

    let mut grade = String::new();

    io::stdin()
        .read_line(&mut grade)
        .expect("Failed to read line");
    let grade: u8 = match grade.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    if grade >=100 {
        println!("Invalid Grade")
    }
    else if grade >= 90 {
        println!("Grade is an A");
    } else if grade >= 80 {
        println!("Grade is a B");
    } else if grade >= 70 {
        println!("Grade is a C");
    } else if grade >= 60 { 
        println!("Grade is D");
    } else {
        println!("Grade is an F");
    }
        
    
//Simple grading scheme
    
}
