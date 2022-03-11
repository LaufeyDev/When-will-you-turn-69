use std::io;

extern crate chrono;

use chrono::prelude::*;

pub fn run(){

    let name = input("What's your name? ").expect("oops");

    println!("Hello {} ! " , name);

    let age = input("What's your age? ")
     .expect("oops")
     .parse:: <i32> ().expect("oops");

    

    let current_year = Utc::now().year();
    let sixty_nine_years = 69 - age + current_year;

    println!("you're {} and You'll turn 69 in {}" , age , sixty_nine_years);
    
    
}

fn input(user_message: &str) -> io::Result<String> {

    use std::io::Write;

    print!("{}" , user_message);
    io::stdout().flush()?;

    let mut reply: String = String::new();

    io::stdin().read_line(&mut reply)?;
    
    Ok(reply.trim_end().to_owned())

   
}