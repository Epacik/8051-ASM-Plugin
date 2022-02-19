use std::borrow::Borrow;

static DOCUMENTATION_DIR: &str = "json_documentation";

#[proc_macro]
pub fn load_documentation(_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(load_docs(proc_macro2::TokenStream::from(_stream)))
}

fn load_docs(_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let lang = _stream.to_string();
    //println!("load docs from {}", lang.clone());

    //checking if directory with documentation is present

    // that variable should point to library's directory
    let project_path_result = std::env::var("CARGO_MANIFEST_DIR");
    if project_path_result.is_err() {
        panic!(
            "Environment variable CARGO_MANIFEST_DIR could not be read\n{}",
            project_path_result.unwrap_err()
        );
    }

    let project_path = project_path_result.unwrap();

    let mut path = project_path;

    if path.ends_with("lsp_server_8051_asm") {
        // rust analyzer sometimes dies because of this macro
        // return quote::quote!{ std::collections::HashMap::new() };
        path.push_str("/");
        path.push_str("load_documentation");
    }

    path.push_str("/");
    path.push_str(DOCUMENTATION_DIR);
    path.push_str("/");
    path.push_str(<std::string::String as Borrow<str>>::borrow(&lang));

    let folders_result = std::fs::read_dir(&path);

    // check if requested language is available
    if folders_result.is_err() {
        panic!("{} was not found in {}", folders_result.unwrap_err(), &path);
    }

    // load folder with selected language
    let docs_folder = folders_result.unwrap();

    let mut items: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();

    //load all files stored within that folder and make a vector of key-value pairs
    for file in docs_folder {
        let file_ref = file.as_ref();

        if file_ref.is_err() {
            eprintln!("ERROR");
            continue;
        }

        let filename = file_ref.unwrap().file_name().into_string().unwrap();

        //println!("loading {}", filename);

        let content = std::fs::read_to_string(file_ref.unwrap().path());

        if content.is_err() {
            eprintln!("ERROR");
            continue;
        }

        //load file as a map
        let docs: Result<
            serde_json::Map<std::string::String, serde_json::Value>,
            serde_json::Error,
        > = serde_json::from_str(content.unwrap().as_str());

        if docs.is_err() {
            eprintln!(
                "Error loading documentation file: {}\nError: {}",
                file.unwrap().path().into_os_string().into_string().unwrap(),
                docs.unwrap_err()
            );
            continue;
        }

        //turn every pair into from a map into something returnable
        for key_value_pair in docs.unwrap() {
            let key = key_value_pair.0;

            let item_res: Result<crate::Documentation, serde_json::Error> =
                serde_json::from_value(key_value_pair.1);

            if item_res.is_err() {
                //TODO: find a way to display that error
                eprintln!("Error parsing documentation item:"); //, item_res.unwrap_err());
                continue;
            }

            let item = item_res.unwrap();

            let detail = item.detail;
            let description = item.description;
            //let syntax = item.syntax;
            let mut affected_flags: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();
            let mut valid_operands:std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();// = item.valid_operands;

            let category_split = filename.split(".").collect::<Vec<&str>>();
            let mut v: Vec<char> = category_split[0].chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            let category: String = v.into_iter().collect();
            let dont_generate_syntax = item.dont_generate_syntax;

            for flag in item.affected_flags {
                let bit = flag.flag;
                let when_set = flag.when_set;
                let when_unset = flag.when_unset;
                affected_flags.push(quote::quote! {(
                    crate::hover_documentation::documentation::Flag {
                        flag: #bit,
                        when_set: std::string::String::from(#when_set),
                        when_unset: std::string::String::from(#when_unset),
                    }
                )});
            }

            for operands in item.valid_operands {
                let mut op: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();
                for operand in operands {
                    let o = operand.operand;
                    let when = operand.when_first_is;
                    op.push(quote::quote! {(
                        crate::hover_documentation::documentation::ValidOperand {
                            operand: #o,
                            when_first_is: #when,
                        }
                    )});
                }
                valid_operands.push(quote::quote!{
                    std::vec::Vec::from([#(#op),*])
                });
            }

            items.push(quote::quote!{ ( std::string::String::from(#key), crate::hover_documentation::documentation::Documentation {
                detail: std::string::String::from(#detail),
                description: std::string::String::from(#description),
                //syntax: std::string::String::from(#syntax),
                affected_flags: std::vec::Vec::from([#(#affected_flags),*]),//std::string::String::from(#affected_flags),
                valid_operands: std::vec::Vec::from([#(#valid_operands),*]),//std::string::String::from(#valid_operands),
                category: std::string::String::from(#category),
                dont_generate_syntax: #dont_generate_syntax,
             })});
        }
    }

    quote::quote! { std::collections::HashMap::from([#(#items),*]) }
}

///
#[allow(dead_code)]
#[derive(serde::Deserialize, Default)]
struct Documentation {
    pub detail: std::string::String,
    pub description: std::string::String,
    pub valid_operands: std::vec::Vec<std::vec::Vec<ValidOperand>>,
    pub affected_flags: std::vec::Vec<Flag>,
    pub dont_generate_syntax: bool,
}
// struct Documentation {
//     pub detail: std::string::String,
//     pub description: std::string::String,
//     pub syntax: std::string::String,
//     pub affected_flags: std::string::String,
//     pub valid_operands: std::string::String,
// }

#[allow(dead_code)]
#[derive(serde::Deserialize, Default)]
struct Flag {
    pub flag: i32,
    pub when_set: std::string::String,
    pub when_unset: std::string::String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Default)]
struct ValidOperand {
    pub operand: i32,
    pub when_first_is: i32,
}

// #[cfg(test)]
// mod tests {
//     use crate::load_docs;
//     use std::str::FromStr;

//     #[test]
//     fn test_doc_loading() {
//         let docs = load_docs(proc_macro2::TokenStream::from_str("english").unwrap());
//         assert_eq!(true, true);
//     }
// }
