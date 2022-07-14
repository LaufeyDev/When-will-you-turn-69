use chrono::prelude::*;
use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    let name = input("What's your name? ");

    if let Ok(name) = name {
        if name.is_empty() {
            println!("Enter a name!");
            exit(1);
        } else {
            println!("Hello {}! ", name);

            let age = input("What's your age? ");

            if let Ok(age) = age {
                if age.is_empty() {
                    println!("Enter an age!");
                    exit(1);
                } else {
                    let age = age.parse::<i32>();
                    if let Ok(age) = age {
                        let current_year = Utc::now().year();
                        if age >= 69 {
                            println!("You are too old to use this tool!");
                            exit(1);
                        }
                        let sixty_nine_years = 69 - age + current_year;

                        println!(
                            "You're {} year(s) old and you'll turn 69 in {}",
                            age, sixty_nine_years
                        )
                    } else {
                        println!("Enter a valid age!");
                        exit(1);
                    }
                }
            }
        }
    } else {
        println!("Something went wrong.");
        exit(1);
    }
}

fn input(user_message: &str) -> io::Result<String> {
    let mut reply: String = String::new();
    print!("{}", user_message);

    io::stdout().flush()?;
    io::stdin().read_line(&mut reply)?;

    Ok(reply.trim_end().to_owned())
}
