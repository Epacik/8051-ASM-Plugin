pub mod lexer_old;
pub mod lexer;
pub mod parser;
pub mod issues;

asm8051_localize::init!();

#[cfg(test)]
mod tests {

    #[test]
    fn t() {
        let string = String::from("Hello, world!");
        println!("{}", string);
    }
}
