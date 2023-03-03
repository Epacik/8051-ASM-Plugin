use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DefaultLocalizer, LanguageLoader, Localizer,
};
use lazy_static::lazy_static;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

lazy_static! {
    static ref LANGUAGE_LOADER: FluentLanguageLoader = {
        let loader: FluentLanguageLoader = fluent_language_loader!();

        // Load the fallback langauge by default so that users of the
        // library don't need to if they don't care about localization.
        loader
            .load_fallback_language(&Localizations)
            .expect("Error while loading fallback language");
    
        loader
    };
}


pub fn change_language(id: &str) {
    localizer().select(&[id.parse().unwrap()]).unwrap();
    asm8051_parser::i18n::change_language(id);
}

pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

pub fn loader() -> &'static FluentLanguageLoader {
    &*LANGUAGE_LOADER
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! localize {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::loader(), $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::loader(), $message_id, $($args), *)
    }};
}
