pub fn change_language(id: &str) {
    crate::change_language(&id);
    asm8051_parser::change_language(&id);
    asm8051_shared::change_language(&id);
}