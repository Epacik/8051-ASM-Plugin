pub mod lexer_old;
pub mod lexer;
pub mod issues;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub fn change_language<S: AsRef<str>>(id: S) {
    let lang = id.as_ref();

    rust_i18n::set_locale(lang);
}
     
//asm8051_localize::init!();

#[cfg(test)]
mod tests {

    #[test]
    fn t() {
        let string = String::from("Hello, world!");
        println!("{}", string);
    }
}
