#[allow(unused_macros)]
#[macro_export]
macro_rules! init {
    () => {
        #[macro_use]
        extern crate rust_i18n;

        i18n!("../locales", fallback = "en");

        pub fn change_language<S: AsRef<str>>(id: S) {
            let lang = id.as_ref();

            rust_i18n::set_locale(lang);
        }
    };
}

