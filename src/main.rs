use locales::t;

use console::style;

use rand::Rng;
use requestty::Question;

fn main() {
    let lang = "en";


    println!("{} {} {}", style("Guess the").black().on_red(), style("number").red().bold().on_black(), style("!").black().on_red());
    let secret_number = rand::thread_rng().gen_range(1..101);
    

    //loop {

        let user_number = Question::int("number")
        .message("Enter your number?")
        .validate(|number, previous_answers| {
            if number > 0 && number <= 100 {
                Ok(())
            } else {
                Err(format!("Number must be higher than 0 and lower than 100 (current number {} )", number))
            }
        })
        .build();
        println!("{:#?}", requestty::prompt_one(user_number));
        println!("{:#?}", number)
        

    //}

    
    //println!(!("out.hello_world", lang);

    
}
