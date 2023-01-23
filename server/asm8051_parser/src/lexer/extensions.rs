use std::borrow::Borrow;

pub(crate) trait Digits {
    fn ends_with_digit(&self) -> bool;
    fn starts_with_digit(&self) -> bool;
    fn is_binary(&self) -> bool;
    fn is_octal(&self) -> bool;
    fn is_decimal(&self) -> bool;
    fn is_hexadecimal(&self) -> bool;
    fn is_number(&self) -> bool;

}
const BIN_NUMS: [&str; 2] = ["0", "1"];
const OCT_NUMS: [&str; 8] = ["0", "1", "2", "3", "4", "5", "6", "7"];
const DEC_NUMS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const HEX_NUMS: [&str; 22] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "A", "B", "C", "D", "E", "F",
    "a", "b", "c", "d", "e", "f"
];

fn ends_with_digit(s: &dyn AsRef<str>) -> bool {
    let s = s.as_ref();

    DEC_NUMS.iter().any(|x| s.ends_with::<&str>(x))
}

fn starts_with_digit(s: &dyn AsRef<str>) -> bool {
    let s = s.as_ref();

    DEC_NUMS.iter().any(|x| s.starts_with::<&str>(x))
}

fn is_bin(s: &dyn AsRef<str>) -> bool {
    let s = s.as_ref();

    if !s.ends_with("B") && !s.ends_with("b") {
        return false;
    }

    let s = &s[..(s.len() - 1)];

    s.chars().all(|x| BIN_NUMS.contains(&(x.to_string().as_str())))
}

fn is_oct(s: &dyn AsRef<str>) -> bool {
    let s = s.as_ref();

    if !s.ends_with("O") && !s.ends_with("o") {
        return false;
    }

    let s = &s[..(s.len() - 1)];

    s.chars().all(|x| OCT_NUMS.contains(&(x.to_string().as_str())))
}

fn is_dec(s: &dyn AsRef<str>) -> bool {
    let s = s.as_ref();

    if !s.ends_with_digit() {
        return false;
    }

    s.chars().all(|x| DEC_NUMS.contains(&(x.to_string().as_str())))
}

fn is_hex(s: &dyn AsRef<str>) -> bool {
    let s = s.as_ref();

    if !s.ends_with("H") && !s.ends_with("h") {
        return false;
    }

    let s = &s[..(s.len() - 1)];

    s.chars().all(|x| HEX_NUMS.contains(&(x.to_string().as_str())))
}


impl Digits for &str {
    fn ends_with_digit(&self) -> bool { ends_with_digit(&self) }

    fn starts_with_digit(&self) -> bool { starts_with_digit(&self) }

    fn is_binary(&self) -> bool { is_bin(&self) }

    fn is_octal(&self) -> bool { is_oct(&self) }

    fn is_decimal(&self) -> bool { is_dec(&self) }

    fn is_hexadecimal(&self) -> bool { is_hex(&self) }

    fn is_number(&self) -> bool {
        is_bin(&self) || is_oct(&self) || is_dec(&self) || is_hex(&self)
    }
}

impl Digits for String {
    fn ends_with_digit(&self) -> bool { ends_with_digit(&self) }

    fn starts_with_digit(&self) -> bool {  starts_with_digit(&self) }

    fn is_binary(&self) -> bool { is_bin(&self) }

    fn is_octal(&self) -> bool { is_oct(&self) }

    fn is_decimal(&self) -> bool { is_dec(&self) }

    fn is_hexadecimal(&self) -> bool { is_hex(&self) }

    fn is_number(&self) -> bool {
        is_bin(&self) || is_oct(&self) || is_dec(&self) || is_hex(&self)
    }
}