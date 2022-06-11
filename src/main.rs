use locales::t;

use console::style;

use rand::Rng;

fn main() {
    let lang = "en";


    println!("{} {} {}", style("Guess the").black().on_red(), style("number").red().bold().on_black(), style("!").black().on_red());
    let secret_number = rand::thread_rng().gen_range(1..101);
    

    loop {
        
    }

    
    //println!(!("out.hello_world", lang);

    
}
