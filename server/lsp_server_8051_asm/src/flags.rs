use bitflags::bitflags;

bitflags! {
    pub struct Kits: u32 {
        const GENERIC_8051 = 1;
        const DSM51        = 2;
    }

    pub struct Locale: u32 {
        const DEFAULT = 1;
        const ENGLISH = 2;
        const POLISH  = 3;
    }
}

#[allow(dead_code)]
impl Locale {
    pub fn lang_name(&self) -> String {
        (match self.bits {
            2 => "en",
            3 => "pl",
            _ => "",
        })
        .to_string()
    }
}
