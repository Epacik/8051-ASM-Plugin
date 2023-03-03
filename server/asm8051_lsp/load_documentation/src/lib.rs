use std::borrow::Borrow;

static DOCUMENTATION_DIR: &str = "json_documentation";

#[proc_macro]
pub fn load_documentation(_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(load_docs(proc_macro2::TokenStream::from(_stream)))
}

fn load_docs(_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let lang = _stream.to_string();

    let mut path = get_project_path();
    add_relative_path_to_language_dir(&mut path, lang);

    let docs_folder = read_documentation_directory(path);

    let mut items: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();

    //load all files stored within that folder and make a vector of key-value pairs
    for file in docs_folder {
        if file.is_err(){
            continue;
        }
        let (file_path, filename) =  read_filename_and_path(&file); 

        let content = match read_file_content(file_path.borrow()) {
            Some(value) => value,
            None => continue,
        };

        let docs = match load_map_from_json(content, file_path.borrow()) {
            Some(value) => value,
            None => continue,
        };

        //turn every pair into from a map into something returnable
        for (key, value) in docs {

            let item = match parse_to_documentation(value, file_path.to_str().unwrap()) {
                Some(value) => value,
                None => continue,
            };

            let detail = item.detail; 
            let description = item.description;
            let affected_flags = parse_affected_flags(item.affected_flags.borrow());
            let valid_operands = parse_valid_operands(item.valid_operands.borrow());
            let category = filename.strip_suffix(".json");
            let dont_generate_syntax = item.dont_generate_syntax;
            let dont_duplicate_in_all_docs = item.dont_duplicate_in_all_docs;
            let prefix = item.prefix;
            let prefix_required = item.prefix_required;
            let label = match item.label {
                Some(value) => quote::quote!(std::option::Option::Some(std::string::String::from(#value))),
                None => quote::quote!(std::option::Option::None),
            };

            for partial_key in key.split(";") { 
                let pkey = partial_key.trim();
                items.push(quote::quote!{ ( std::string::String::from(#pkey), crate::hover::Documentation {
                    detail: std::string::String::from(#detail),
                    description: std::string::String::from(#description),
                    affected_flags: std::vec::Vec::from([#(#affected_flags),*]),
                    valid_operands: std::vec::Vec::from([#(#valid_operands),*]), 
                    category: std::string::String::from(#category),
                    dont_generate_syntax: #dont_generate_syntax,
                    dont_duplicate_in_all_docs: #dont_duplicate_in_all_docs,
                    full_key: std::string::String::from(#key),
                    prefix: std::string::String::from(#prefix),
                    prefix_required: #prefix_required,
                    label: #label,
                 })});
            }

            
        }
    }

    quote::quote! { std::collections::HashMap::from([#(#items),*]) }
}

#[inline(always)]
fn get_project_path() -> String {
    let project_path_result = std::env::var("CARGO_MANIFEST_DIR");
    if project_path_result.is_err() {
        panic!(
            "Environment variable CARGO_MANIFEST_DIR could not be read\n{}",
            project_path_result.unwrap_err()
        );
    }
    let project_path = project_path_result.unwrap();
    project_path 
}

#[inline(always)]
fn add_relative_path_to_language_dir(path: &mut String, lang: String) {
    if path.ends_with("asm8051_lsp") {
        path.push_str("/load_documentation");
    }
    path.push_str("/");
    path.push_str(DOCUMENTATION_DIR);
    path.push_str("/");
    path.push_str(<std::string::String as Borrow<str>>::borrow(&lang));
}

#[inline(always)]
fn read_documentation_directory(path: String) -> std::fs::ReadDir {
    let folders_result = std::fs::read_dir(&path);
    // check if requested language is available
    if folders_result.is_err() {
        panic!("{} was not found in {}", folders_result.unwrap_err(), &path);
    }
    // load folder with selected language
    let docs_folder = folders_result.unwrap();
    docs_folder
}

#[inline(always)]
fn read_filename_and_path(file: &Result<std::fs::DirEntry, std::io::Error>) -> (std::path::PathBuf, String) {
    let file_ref = file.as_ref();
    let filename = file_ref.unwrap().file_name().into_string().unwrap();
    let path = file_ref.unwrap().path();
    (path, filename)
}

#[inline(always)]
fn read_file_content(file_path: &std::path::PathBuf) -> Option<String> {
    let content = std::fs::read_to_string(file_path);
    if content.is_err() {
        eprintln!("Error reading files content");
        return None;
    }
    let content_str = content.unwrap();
    Some(content_str)
}

#[inline(always)]
fn load_map_from_json(content: String, file_path: &std::path::PathBuf) -> Option<serde_json::Map<String, serde_json::Value>> {
    let docs_result: Result<serde_json::Map<std::string::String, serde_json::Value>, serde_json::Error> =
        serde_json::from_str(content.as_str());
    if docs_result.is_err() {
        eprintln!(
            "Error loading documentation file: {}\nError: {}",
            file_path.as_os_str().to_str().unwrap_or_default(),
            docs_result.unwrap_err()
        );
        return None;
    }
    let docs = docs_result.unwrap();
    Some(docs)
}

#[inline(always)]
fn parse_to_documentation(value: serde_json::Value, filepath: &str) -> Option<Documentation> {
    let item_res: Result<crate::Documentation, serde_json::Error> =
        serde_json::from_value(value);
    if item_res.is_err() {
        // find a way to display that error
        let error =  item_res.unwrap_err();
        let category = match error.classify() {
            serde_json::error::Category::Io     => "IO",
            serde_json::error::Category::Syntax => "Syntax",
            serde_json::error::Category::Data   => "Data",
            serde_json::error::Category::Eof    => "End of file",
        };
        let line = error.line();
        let column = error.column();
        let msg = error.to_string();

        eprintln!("Error parsing documentation item:\nCategory: {}\nLine: {}, Column: {}\nMessage: {}\nFile {}\n\n",
         category, line, column, msg, filepath);
        return None;
    }
    let item = item_res.unwrap();
    Some(item)
}

#[inline(always)]
fn parse_affected_flags(flags: &std::vec::Vec<Flag>) -> Vec<proc_macro2::TokenStream> {
    let mut affected_flags: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();
    for flag in flags {
        
        let bit = flag.flag;
        let when_set = flag.when_set.clone();
        let when_unset = flag.when_unset.clone();

        affected_flags.push(quote::quote! {(
            crate::hover::Flag {
                flag: #bit,
                when_set: std::string::String::from(#when_set),
                when_unset: std::string::String::from(#when_unset),
            }
        )});
    }
    affected_flags
}

#[inline(always)]
fn parse_valid_operands(vo: &Vec<Vec<ValidOperand>>) -> Vec<proc_macro2::TokenStream> {
    let mut valid_operands:std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();
    for operands in vo {
        let mut op: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();
        for operand in operands {
            let o = operand.operand;
            let when = operand.when_first_is;
            op.push(quote::quote! {(
                crate::hover::ValidOperand {
                    operand: #o,
                    when_first_is: #when,
                }
            )});
        }
        valid_operands.push(quote::quote!{
            std::vec::Vec::from([#(#op),*])
        });
    }
    valid_operands
}
#[allow(dead_code)]
#[derive(serde::Deserialize, Default, Clone, Debug)]
struct Documentation {
    pub detail: std::string::String,
    pub description: std::string::String,
    pub valid_operands: std::vec::Vec<std::vec::Vec<ValidOperand>>,
    pub affected_flags: std::vec::Vec<Flag>,
    pub dont_generate_syntax: bool,
    pub dont_duplicate_in_all_docs: bool,
    pub prefix: std::string::String,
    pub prefix_required: bool,
    pub label: Option<String>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Default, Clone, Debug)]
struct Flag {
    pub flag: i32,
    pub when_set: std::string::String,
    pub when_unset: std::string::String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Default, Clone, Debug)]
struct ValidOperand {
    pub operand: i32,
    pub when_first_is: i32,
}
