use console::style;

use emojicon::Emojicon;
use inquire::error::InquireError;
use inquire::CustomType;
use inquire::Select;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
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
    let mut score = 0;
    let mut best_try = 10000; // Big, very big number :D
    let mut played = false;
    println!("Hi, Gamer! {}", emojicon.get_by_name("wave").unwrap().collect::<Vec<_>>()[0]);
    loop {
        // START: Game

        let options: Vec<&str> = vec!["Start a game", "Quit","Rules", "About"];

        let continue_game: Result<&str, InquireError> =
            Select::new("What do you want to do ?", options).prompt();

        match continue_game {
            Ok(choice) => {
                match choice {
                    "Start a game" => {println!(
                        "Start a new game {}",
                        emojicon.get_by_name("game").unwrap().collect::<Vec<_>>()[0]);

                        println!(
                            "Generating the secret number ... {}",
                            emojicon.get_by_name("key").unwrap().collect::<Vec<_>>()[0]
                        );
                        let secret_number = rand::thread_rng().gen_range(1..101);
                        let mut try_user = 0;
                        loop {
                            let user_number = CustomType::<u32>::new("Whats your number ?")
                                .with_formatter(&|i| format!("{:.2}", i))
                                .with_error_message("Please type a valid number")
                                .with_help_message(format!("Write a number ({})", try_user).as_str())
                                .with_placeholder("number")
                                .prompt();
                
                            match user_number {
                                Ok(_) => println!(
                                    "Check if match.... {}",
                                    emojicon.get_by_name("fire").unwrap().collect::<Vec<_>>()[0]
                                ),
                                Err(_) => println!(
                                    "Good bye {}",
                                    emojicon.get_by_name("wave").unwrap().collect::<Vec<_>>()[0]
                                ),
                            }
                
                            try_user = try_user + 1;
                
                            match user_number.unwrap().cmp(&secret_number) {
                                Ordering::Less => {
                                    println!(
                                        "It's higher! {}",
                                        emojicon
                                            .get_by_name("thumbsup")
                                            .unwrap()
                                            .collect::<Vec<_>>()[0]
                                    );
                                }
                                Ordering::Greater => {
                                    println!(
                                        "It's lower {}",
                                        emojicon
                                            .get_by_name("thumbsdown")
                                            .unwrap()
                                            .collect::<Vec<_>>()[0]
                                    );
                                }
                                Ordering::Equal => {
                                    println!(
                                        "You won in {}! {}",
                                        try_user,
                                        emojicon.get_by_name("trophy").unwrap().collect::<Vec<_>>()[0]
                                    );
                
                                    if try_user < best_try {
                                        best_try = try_user;
                                        println!("It's your new goals to beat. Good luck!");
                                    }
                                    score = score + 1;
                                    played = true;
                                    break;
                                }
                            }
                
                            
                        }
                
                        println!(
                            "You have {} {}",
                            score,
                            emojicon.get_by_name("trophy").unwrap().collect::<Vec<_>>()[0]
                        );
                
                        // END: GAME
                    },
                    "Quit" => {
                        if played == false {
                            println!("Oh noh! You didn't play yet. {}", emojicon.get_by_name("cry").unwrap().collect::<Vec<_>>()[0]);
                        }
                        else {
                            println!("Your best score is {}", best_try);
                        }
                        break;
                    },
                    "Rules" => {
                        println!("The computer generates a random number. Then you have to guess this number. The goal is to find the secret number as little as possible. Good luck!");
                    },
                    "About" => {
                        println!("This game was created by 0xMRTT to learn rust. It's a free software. You can find source on https://github.com/0xMRTT/guessing_game.")
                    },
                    &_ => {
                        // Same as quit
                        println!("Your best score is {}", best_try);
                        break;
                    }
                }
            }
            Err(_) => println!("There was an error, please try again"),
        }

        

        
    }

    println!(
        "Good bye {}",
        emojicon.get_by_name("wave").unwrap().collect::<Vec<_>>()[0]
    );
}
