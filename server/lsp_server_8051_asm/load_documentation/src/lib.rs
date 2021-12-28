use std::borrow::Borrow;


// let's just include the whole directory of library!
// for some reason this macro always shows as error in rust-analyzer
static PROJECT_DIR: include_dir::Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR");
static DOCUMENTATION_DIR: &str = "json_documentation";

#[proc_macro]
pub fn load_documentation(_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(load_docs(proc_macro2::TokenStream::from(_stream)))
}

fn load_docs(_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    println!("load docs");

    //checking if directory with documentation is present
    let lang = _stream.to_string();
    if !PROJECT_DIR.contains(DOCUMENTATION_DIR){
        panic!("{} was not found in {}", DOCUMENTATION_DIR, PROJECT_DIR.path().to_str().unwrap());
    }

    // get open folder with documentation
    let all_docs_folder = PROJECT_DIR.get_dir(DOCUMENTATION_DIR).unwrap();

    let mut path = std::string::String::from(DOCUMENTATION_DIR);
    path.push_str("/");
    path.push_str(<std::string::String as Borrow<str>>::borrow(&lang));

    // check if requested language is available
    if !all_docs_folder.contains(<std::string::String as Borrow<str>>::borrow(&path)){
        let items = all_docs_folder.dirs().map(|d| d.path().to_str().unwrap());
        
        //list contents of folder
        let mut string = std::string::String::new();
        for i in items {
            string.push_str(i);
            string.push_str("\n");
        }

        panic!("{} was not found in {}/{}\n{}", <std::string::String as Borrow<str>>::borrow(&lang),PROJECT_DIR.path().to_str().unwrap(),  all_docs_folder.path().to_str().unwrap(), string);
    }

    // load folder with selected language
    let docs_folder = all_docs_folder.get_dir(path.as_str()).unwrap();
    
    let mut items: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();

    //load all files stored within that folder and make a vector of key-value pairs
    for file in docs_folder.files() {
        println!("loading {}", file.path().to_str().unwrap());
        let content = file.contents_utf8();
        if content.is_none() {
            eprintln!("ERROR");
            continue;
        }

        //load file as a map
        let docs : Result<serde_json::Map<std::string::String, serde_json::Value>, serde_json::Error> = serde_json::from_str(content.unwrap());

        if docs.is_err() {
            eprintln!("Error loading documentation file: {}\nError: {}", 
            file.path().file_name().unwrap_or_default().to_str().unwrap(), 
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


#[cfg(test)]
mod tests {
    use crate::load_docs;
    use std::str::FromStr;

    #[test]
    fn test_doc_loading() {
        let docs = load_docs(proc_macro2::TokenStream::from_str("english").unwrap());
        assert_eq!(true, true);
    }
}
