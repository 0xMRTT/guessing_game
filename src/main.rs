use locales::t;

fn main() {
    let lang = "en";
    println!("{}", t!("err.hello", lang));
}
