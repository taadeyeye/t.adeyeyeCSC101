//rust program to input age 

use std::io;
 fn main() {
    println!("\nstudent information management system!");

    //input name
    println!("\nplease enter your name.");
    let mut name = String::new(); 
    io:: stdin()
    .read_line(&mut name)
    .expect("Failed to read input");
    println!("your name is {}",name );

    //input age
    println!("\nenter your age", );
    let mut age = String::new();
    io::stdin() .read_line(&mut age) .expect("failed to read input");
    let age :i32 = age.trim().parse().expect("input is not an integer");
    println!("your age is {}",age );


}
