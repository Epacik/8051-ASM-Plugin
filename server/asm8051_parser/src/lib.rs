pub mod lexer_old;
pub mod lexer;
pub mod issues;

#[macro_use]
extern crate asm8051_localize;

#[cfg(test)]
mod tests {

    #[test]
    fn t() {
        let string = String::from("Hello, world!");
        println!("{}", string);
    }
}
