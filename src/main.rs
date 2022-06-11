use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main() {
    println!("{}", t!("hello"));
}
