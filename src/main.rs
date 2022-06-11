use locales::t;

use console::style;

use rand::Rng;
use requestty::Question;
use inquire::CustomType;
use std::cmp::Ordering;

fn main() {
    let lang = "en";


    println!("{} {} {}", style("Guess the").black().on_red(), style("number").red().bold().on_black(), style("!").black().on_red());
    let secret_number = rand::thread_rng().gen_range(1..101);
    

    loop {
        let user_number = CustomType::<u32>::new("Whats your number ?")
            .with_formatter(&|i| format!("{:.2}", i))
            .with_error_message("Please type a valid number")
            .with_help_message("Type the number")
            .with_placeholder("number")
            .prompt();

        match user_number {
            Ok(_) => println!("Check if match...."),
            Err(_) => println!("ERROR"),
        }

        match user_number.unwrap().cmp(&secret_number) {
            Ordering::Less => println!("It's higher!"),
            Ordering::Greater => println!("It's lower"),
            Ordering::Equal => {
                println!("You won !");
                break;
            }
        }

    }

    
    //println!(!("out.hello_world", lang);

    
}
