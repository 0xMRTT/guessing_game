use locales::t;

fn main() {
    let lang = "en";
    println!("{}", t!("out.hello_world", lang));
}
