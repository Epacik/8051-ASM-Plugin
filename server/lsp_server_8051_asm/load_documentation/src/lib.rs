use std::borrow::Borrow;

use include_dir::{Dir};
use proc_macro2::TokenStream;

static PROJECT_DIR: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR");
static DOCUMENTATION_DIR: &str = "json_documentation";

#[proc_macro]
pub fn load_documentation(_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(load_docs(TokenStream::from(_stream)))
}

fn load_docs(_stream: TokenStream) -> TokenStream {
    if !PROJECT_DIR.contains(DOCUMENTATION_DIR){
        panic!("{} was not found in proc_macro lib dir", DOCUMENTATION_DIR);
    }

    let docs_folder = PROJECT_DIR.get_dir(DOCUMENTATION_DIR).unwrap();

    let mut str = String::from("std::collections::HashMap::from([");
    let mut index = 0;
    let dirs = docs_folder.dirs();
    let dirs_count = docs_folder.dirs().count();
    for entry in dirs {
        str.push_str("(\"");
        str.push_str(entry.path().file_name().unwrap().to_str().unwrap().to_uppercase().as_str());
        str.push_str("\", \"");
        str.push_str(_stream.to_string().as_str());
        if index + 1 == dirs_count {
            str.push_str("\")\n");
        }
        else {
            str.push_str("\"),\n");
        }

        index += 1;
    }
    str.push_str("])");

    str.as_str().parse().unwrap()
}



#[cfg(test)]
mod tests {
    use crate::load_docs;
    use std::str::FromStr;

    #[test]
    fn test_doc_loading() {
        load_docs(proc_macro2::TokenStream::from_str("fn foo() {}").unwrap());
        assert_eq!(true, true);
    }
}
