mod types;
mod debugging;
use types::*;
use std::{borrow::Borrow, collections::HashMap};

pub fn load_docs(_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let lang = _stream.to_string();

    let path = get_project_path();
    let shared_path = add_relative_path_to_language_dir(&path, ".shared".to_string());
    let path = add_relative_path_to_language_dir(&path, lang);

    let files: Vec::<FileDesctiption> = get_files(&path, &shared_path);

    let mut items: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();

    //load all files stored within that folder and make a vector of key-value pairs
    for file in files {
        
        let main_content = if let Some(path) = file.main_path() {
            read_file_content(&path)
        } 
        else { 
            None
        };

        let shared_content = if let Some(path) = file.shared_path() {
            read_file_content(&path)
        } 
        else { 
            None
        };

        if main_content == None && shared_content == None {
            continue;
        }

        let docs = load_map_from_json(&main_content, &shared_content, &file);

        //turn every pair into from a map into something returnable
        for (key, value) in docs {

            if value.main == None && value.shared == None {
                continue;
            }

            let item = parse_to_documentation(&value, &file);

            let detail = item.detail.clone(); 
            let description = item.description.clone();
            let affected_flags = parse_affected_flags(item.affected_flags.borrow());
            let valid_operands = parse_valid_operands(item.valid_operands.borrow());
            let category = file.filename.strip_suffix(".json").unwrap_or_default();
            let dont_generate_syntax = item.dont_generate_syntax;
            let dont_duplicate_in_all_docs = item.dont_duplicate_in_all_docs;
            let prefix = item.prefix.clone();
            let prefix_required = item.prefix_required;
            let label = match item.label.clone() {
                Some(value) => quote::quote!(std::option::Option::Some(std::string::String::from(#value))),
                None => quote::quote!(std::option::Option::None),
            };
            let addressing_modes = parse_addressing_modes(&item);
            let stack_space_needed = match item.stack_space_needed {
                Some(s) => if s == 0 {
                    quote::quote!(std::option::Option::None)
                } 
                else {
                    quote::quote!(std::option::Option::Some(#s))
                },
                None => quote::quote!(std::option::Option::None),
            };
            let used_registers = parse_possible_registers(&item.used_registers);
            let changed_registers = parse_possible_registers(&item.changed_registers);

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
                    addressing_modes: std::vec::Vec::from([#(#addressing_modes),*]),
                    stack_space_needed: #stack_space_needed,
                    used_registers: std::vec::Vec::from([#(#used_registers),*]),
                    changed_registers: std::vec::Vec::from([#(#changed_registers),*])
                 })});
            }
        }
    }

    quote::quote! { std::collections::HashMap::from([#(#items),*]) }
}


fn get_files(main_path: &String, shared_path: &String) -> Vec<FileDesctiption> {
    let main_folder = read_documentation_directory(&main_path);
    let shared_folder = read_documentation_directory(&shared_path);

    let mut files: Vec<(String, bool, bool)> = vec![];

    for file in main_folder {
        let file = match file {
            Ok(f) => f,
            Err(_) => continue,
        };
        let (_, filename) =  read_filename_and_path(&file);

        if !files.iter().any(|x| &x.0 == &filename) {
            files.push((filename, true, false));
        }
    }

    for file in shared_folder {

        let file = match file {
            Ok(f) => f,
            Err(_) => continue,
        };

        let (_, filename) =  read_filename_and_path(&file);

        if !files.iter().any(|x| &x.0 == &filename) {
            files.push((filename, false, true));
        }
        else {
            let pos = files.iter().position(|x| &x.0 == &filename);
            if let Some(pos) = pos {
                files.remove(pos);
            }

            files.push((filename, true, true));
        }
    }

    let mut result = Vec::<FileDesctiption>::new();

    for (name, in_main, in_shared) in files {
        result.push(FileDesctiption::new(
            name,
            if in_shared { Some(shared_path.clone())} else { None },
            if in_main { Some(main_path.clone())} else { None }));
    }


    result

}

#[inline(always)]
fn get_project_path() -> String {
    let project_path_result = std::env::var("CARGO_MANIFEST_DIR");
    match project_path_result {
        Ok(path) => path,
        Err(_) => {
            match std::env::current_dir() {
                Ok(dir) => {
                    match dir.as_os_str().to_str() {
                        Some(path) => String::from(path),
                        None => panic!("Could not read a directory containing documentation"),
                    }
                },
                Err(_) => panic!("Could not read a directory containing documentation"),
            }
        },
    }
}

#[inline(always)]
fn add_relative_path_to_language_dir(path: &String, lang: String) -> String {
    let mut path = path.clone();
    if path.ends_with("asm8051_lsp") {
        path.push_str("/macros/load_documentation");
    }
    if path.ends_with("load_documentation_internal"){
    }
    path.push_str("/json_documentation/");
    path.push_str(<std::string::String as Borrow<str>>::borrow(&lang));

    path
}

#[inline(always)]
fn read_documentation_directory(path: &String) -> std::fs::ReadDir {
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
fn read_filename_and_path(file: &std::fs::DirEntry) -> (std::path::PathBuf, String) {

    let filename = file.file_name().into_string().unwrap();
    let path = file.path();
    (path, filename)
}

#[inline(always)]
fn read_file_content<P: AsRef<std::path::Path>>(path: P) -> Option<String> {
    let content = std::fs::read_to_string(path.as_ref());

    match content {
        Ok(c) => Some(c),
        Err(_err) => {
            eprintln!("Error reading files content, path: '{}'", path.as_ref().to_str().unwrap());
            None
        },
    }
}

type JsonResult = Result<serde_json::Map<std::string::String, serde_json::Value>, serde_json::Error>;
#[inline(always)]
fn load_map_from_json(main_content: &Option<String>, shared_content: &Option<String>,  file: &FileDesctiption) 
    -> HashMap<String, Values> {

    let main_values = if let Some(content) = main_content {
        let result: JsonResult = serde_json::from_str(content.as_str());
        match result {
            Ok(val) => Some(val),
            Err(err) => {
                eprintln!(
                    "Error loading documentation file: {}\nError: {}",
                    file.main_path().unwrap_or_default(),
                    err
                );
                
                None
            },
        }
    }
    else {
        None
    };

    let shared_values = if let Some(content) = shared_content {
        let result: JsonResult = serde_json::from_str(content.as_str());
        match result {
            Ok(val) => Some(val),
            Err(err) => {
                eprintln!(
                    "Error loading documentation file: {}\nError: {}",
                    file.shared_path().unwrap_or_default(),
                    err
                );
                
                None
            },
        }
    }
    else {
        None
    };

    let mut map: HashMap<String, Values> = HashMap::new();

    if let Some(values) = main_values {
        for (key, value) in values {
            if !map.iter().any(|(m_key, _)| m_key == &key) {
                map.insert(key.clone(), Values {main: Some(value), shared: None});
            }
        }
    }

    if let Some(values) = shared_values {
        for (key, value) in values {
            let value = value.clone();
            if !map.iter().any(|(m_key, _)| &m_key == &&key) {
                map.insert(key.clone(), Values {main: None, shared: Some(value.clone())});
            }
            else {

                let item = map.get(&key).unwrap().clone();

                map.remove(&key);
                map.insert(key.clone(), Values {main: item.main, shared: Some(value)});
            }
        }
    }

    map
}

#[inline(always)]
fn parse_to_documentation(value: &Values, file: &FileDesctiption) -> Documentation {
    let mut doc = Documentation::default();

    if let Some(value) = &value.main {
        match serde_json::from_value::<MainDocumentationElement>(value.clone()) {
            Ok(res) => {
                doc.detail = res.detail;
                doc.description = res.description;
                doc.label = res.label;
                doc.prefix = res.prefix;
                let affected_flags: Vec<Flag> = res.affected_flags.iter()
                    .map(|(flag, desc)| Flag::new(
                        flag.clone(), 
                        desc.when_set.clone(), 
                        desc.when_unset.clone()))
                    .collect::<Vec<Flag>>();

                doc.affected_flags = affected_flags;

            },
            Err(error) => {
                eprintln!("Main");
                print_json_parse_error(error, file.main_path().unwrap_or_default());
            },
        };
    }

    if let Some(value) = &value.shared {
        match serde_json::from_value::<SharedDocumentationElement>(value.clone()) {
            Ok(res) => {
                doc.valid_operands = res.valid_operands;
                doc.dont_duplicate_in_all_docs = res.dont_duplicate_in_all_docs;
                doc.dont_generate_syntax = res.dont_generate_syntax;
                doc.prefix_required = res.prefix_required;
                doc.addressing_modes = res.addressing_modes;

                for flag in res.affected_flags {
                    if !doc.affected_flags.iter().any(|x| &x.flag == &flag) {
                        doc.affected_flags.push(Flag::new(flag, String::new(), String::new()));
                    }
                }

                doc.stack_space_needed = res.stack_space_needed;
                doc.used_registers = res.used_registers;
                doc.changed_registers = res.changed_registers;

            },
            Err(error) => {
                eprintln!("Shared");
                print_json_parse_error(error, file.shared_path().unwrap_or_default());
            },
        };
    }
    
    doc
}

fn print_json_parse_error(error: serde_json::Error, file: String) {
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
     category, line, column, msg, file);
}

#[inline(always)]
fn parse_affected_flags(flags: &std::vec::Vec<Flag>) -> Vec<proc_macro2::TokenStream> {
    let mut affected_flags: std::vec::Vec<proc_macro2::TokenStream> = std::vec::Vec::new();
    for flag in flags {

        let mut fl = String::from("crate::hover::FlagType::");
        fl.push_str(capitalize(&flag.flag).as_str());
        
        let fl: proc_macro2::TokenStream = fl.parse().unwrap();
        let when_set = flag.when_set.clone();
        let when_unset = flag.when_unset.clone();

        affected_flags.push(quote::quote! {(
            crate::hover::Flag {
                flag: #fl,
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

            let mut o = String::from("crate::hover::PossibleOperand::");
            o.push_str(capitalize(&operand.operand).as_str());

            let o: proc_macro2::TokenStream  = o.parse().unwrap();

            let mut when = String::from("crate::hover::PossibleOperand::");
            when.push_str(capitalize(&operand.when_first_is).as_str());
            let when: proc_macro2::TokenStream  = when.parse().unwrap();

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

fn parse_addressing_modes(item: &Documentation) -> Vec<proc_macro2::TokenStream> {

    let modes = match &item.addressing_modes {
        Some(am) => am,
        None => return vec![],
    };

    let mut addressing_modes: Vec<proc_macro2::TokenStream> = Vec::new();

    for mode in modes {
        let mut m = String::from("crate::hover::AddressingMode::");
        m.push_str(capitalize(&mode).as_str());

        let m: proc_macro2::TokenStream = m.parse().unwrap();

        addressing_modes.push(m);
    }
 
    addressing_modes
}


fn parse_possible_registers(registers: &Option<Vec<String>>) -> Vec<proc_macro2::TokenStream> {
    let registers = match registers {
        Some(reg) => reg,
        None => return vec![],
    };

    let mut result = Vec::<proc_macro2::TokenStream>::new();

    for register in registers {
        let mut m = String::from("crate::hover::PossibleRegister::");

        m.push_str(capitalize(&register).as_str());

        let m: proc_macro2::TokenStream = m.parse().unwrap();

        result.push(m);
    }

    result
}

fn capitalize<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


