use locales::t;
use std::thread;
use std::time::Duration;

use console::Term;
use console::style;
use promptly::{prompt, prompt_default, prompt_opt};


fn main() {
    let lang = "en";


    let term = Term::stdout();
    term.write_line(&t!("out.hello_world", lang).to_string());
    thread::sleep(Duration::from_millis(2000));
    term.clear_line();

    println!("This is {} neat", style("quite").blue());

    
}
