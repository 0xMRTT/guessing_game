use gettextrs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the name of the .mo file to use.
    textdomain("guessing_game")?;
    // Ask gettext for UTF-8 strings. THIS CRATE CAN'T HANDLE NON-UTF-8 DATA!
    bind_textdomain_codeset("guessing_game", "UTF-8")?;

    // You could also use `TextDomain` builder which calls `textdomain` and
    // other functions for you:
    //
    // TextDomain::new("hellorust").init()?;

    // `gettext()` simultaneously marks a string for translation and translates
    // it at runtime.
    println!("Translated: {}", gettext("Hello, world!"));

    // gettext supports plurals, i.e. you can have different messages depending
    // on the number of items the message mentions. This even works for
    // languages that have more than one plural form, like Russian or Czech.
    println!("Singular: {}", ngettext("One thing", "Multiple things", 1));
    println!("Plural: {}", ngettext("One thing", "Multiple things", 2));

    // gettext de-duplicates strings, i.e. the same string used multiple times
    // will have a single entry in the PO and MO files. However, the same words
    // might have different meaning depending on the context. To distinguish
    // between different contexts, gettext accepts an additional string:
    println!("With context: {}", pgettext("This is the context", "Hello, world!"));
    println!(
        "Plural with context: {}",
        npgettext("This is the context", "One thing", "Multiple things", 2));

    Ok(())
}