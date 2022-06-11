use locales::t;

use console::style;

use rand::Rng;
use inquire::CustomType;
use std::cmp::Ordering;
use emojicon::Emojicon;

fn main() {
    let lang = "en";
    let emojicon = Emojicon::new();

    println!("{}", style("    
======================================================================================================

    .--.                            .-. .-.                                .-.                .-.
    : .--'                          .' `.: :                                : :                : :
    : : _ .-..-. .--.  .--.  .--.   `. .': `-.  .--.   ,-.,-..-..-.,-.,-.,-.: `-.  .--. .--.   : :
    : :; :: :; :' '_.'`._-.'`._-.'   : : : .. :' '_.'  : ,. :: :; :: ,. ,. :' .; :' '_.': ..'  :_;
    `.__.'`.__.'`.__.'`.__.'`.__.'   :_; :_;:_;`.__.'  :_;:_;`.__.':_;:_;:_;`.__.'`.__.':_;    :_;
                                                                                                  
======================================================================================================                                                                                                                                                                         
   ").bold().red());

    println!("Generating the secret number ... {}", emojicon.get_by_name("key").unwrap().collect::<Vec<_>>()[0]);
    let secret_number = rand::thread_rng().gen_range(1..101);
    

    loop {
        let user_number = CustomType::<u32>::new("Whats your number ?")
            .with_formatter(&|i| format!("{:.2}", i))
            .with_error_message("Please type a valid number")
            .with_help_message("Type the number")
            .with_placeholder("number")
            .prompt();

        match user_number {
            Ok(_) => println!("Check if match.... {}", emojicon.get_by_name("fire").unwrap().collect::<Vec<_>>()[0]),
            Err(_) => println!("Good bye {}", emojicon.get_by_name("wave").unwrap().collect::<Vec<_>>()[0]),
        }

        match user_number.unwrap().cmp(&secret_number) {
            Ordering::Less => println!("It's higher! {}", emojicon.get_by_name("thumbsup").unwrap().collect::<Vec<_>>()[0]),
            Ordering::Greater => println!("It's lower {}", emojicon.get_by_name("thumbsdown").unwrap().collect::<Vec<_>>()[0]),
            Ordering::Equal => {
                println!("You won ! {}", emojicon.get_by_name("trophy").unwrap().collect::<Vec<_>>()[0]);
                break;
            }
        }

    }

    println!("Good bye {}", emojicon.get_by_name("wave").unwrap().collect::<Vec<_>>()[0]);

    
    //println!(!("out.hello_world", lang);

    
}
