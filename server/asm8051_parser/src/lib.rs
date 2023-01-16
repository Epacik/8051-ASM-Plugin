pub mod lexer_old;
pub mod lexer;
pub(self) mod extensions;

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn t() {
        assert!(true);
    }
}
