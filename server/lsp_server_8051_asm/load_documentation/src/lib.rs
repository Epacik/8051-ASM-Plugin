use std::borrow::Borrow;


static DOCUMENTATION_DIR: &str = "json_documentation";

#[proc_macro]
pub fn load_documentation(_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(load_docs(proc_macro2::TokenStream::from(_stream)))
}

fn load_docs(_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let lang = _stream.to_string();
    println!("load docs from {}", lang.clone());

    //checking if directory with documentation is present

    // that variable should point to library's directory
    let project_path_result = std::env::var("CARGO_MANIFEST_DIR");
    if project_path_result.is_err(){
        panic!("Environment variable CARGO_MANIFEST_DIR could not be read\n{}", project_path_result.unwrap_err());
    }

    let project_path = project_path_result.unwrap();


    


    let mut path = project_path;

    if path.ends_with("lsp_server_8051_asm") {
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

        println!("loading {}", file_ref.unwrap().file_name().into_string().unwrap());

        let content = std::fs::read_to_string(file_ref.unwrap().path());

        if content.is_err() {
            eprintln!("ERROR");
            continue;
        }

        //load file as a map
        let docs : Result<serde_json::Map<std::string::String, serde_json::Value>, serde_json::Error> = serde_json::from_str(content.unwrap().as_str());

        if docs.is_err() {
            eprintln!("Error loading documentation file: {}\nError: {}", 
            file.unwrap().path().into_os_string().into_string().unwrap(), 
            docs.unwrap_err());
            continue;
        }

        //turn every pair into from a map into something returnable
        for key_value_pair in docs.unwrap() {
            
            let key = key_value_pair.0;
            
            let item_res: Result<crate::Documentation, serde_json::Error> = serde_json::from_value(key_value_pair.1);

            if item_res.is_err(){
                //TODO: find a way to display that error
                eprintln!("Error parsing documentation item:");//, item_res.unwrap_err());
                continue;
            }

            let item = item_res.unwrap();

            let detail = item.detail;
            let description = item.description;
            let syntax = item.syntax;
            let affected_flags = item.affected_flags;
            let valid_operands = item.valid_operands;

            items.push(quote::quote!{ ( std::string::String::from(#key), crate::hover_documentation::documentation::Documentation {
                detail: std::string::String::from(#detail),
                description: std::string::String::from(#description),
                syntax: std::string::String::from(#syntax),
                affected_flags: std::string::String::from(#affected_flags),
                valid_operands: std::string::String::from(#valid_operands),
             })});
        }
    }

    quote::quote!{ std::collections::HashMap::from([#(#items),*]) }
}

///
#[allow(dead_code)]
#[derive(serde::Deserialize, Default)]
struct Documentation {
    pub detail: std::string::String,
    pub description: std::string::String,
    pub syntax: std::string::String,
    pub affected_flags: std::string::String,
    pub valid_operands: std::string::String,
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
