pub(crate) trait EndsWithDigit {
    fn ends_with_digit(&self) -> bool;
}

fn ends_with_digit(s: &dyn AsRef<str>) -> bool {
    let s = s.as_ref();

    s.ends_with('0') ||
    s.ends_with('1') ||
    s.ends_with('2') ||
    s.ends_with('3') ||
    s.ends_with('4') ||
    s.ends_with('5') ||
    s.ends_with('6') ||
    s.ends_with('7') ||
    s.ends_with('8') ||
    s.ends_with('9')
}

impl EndsWithDigit for &str {
    fn ends_with_digit(&self) -> bool {
        ends_with_digit(&self)
    }
}

impl EndsWithDigit for String {
    fn ends_with_digit(&self) -> bool {
        ends_with_digit(&self)
    }
}