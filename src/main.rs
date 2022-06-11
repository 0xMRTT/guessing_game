use locales::t;

fn main() {
    let lang = "de";
    println!("{}", t!("out.hello_world", lang));
}
