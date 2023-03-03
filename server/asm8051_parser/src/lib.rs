pub mod lexer_old;
pub mod lexer;
pub mod issues;
pub mod i18n;

#[allow(dead_code)]
fn main() {
    
}

#[cfg(test)]
mod tests {

    #[test]
    fn t() {
        let string = String::from("Hello, world!");
        println!("{}", string);
    }
}
