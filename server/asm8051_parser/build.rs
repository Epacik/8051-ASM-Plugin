use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_dir = Path::new(&cargo_manifest_dir);
    let locales = crate_dir.join("locales");

    if !Path::exists(locales.as_path()) {
        fs::create_dir(locales.clone()).unwrap();
    }

    let source = crate_dir.join("..").join("locales").join("app.yaml");
    let target = locales.clone().join("app.yaml");

    if Path::exists(target.as_path()) {
        fs::remove_file(target.as_path()).unwrap();
    }

    if let Err(err) = fs::copy(source.clone(), target.clone()) {
        println!(
            "cargo:error=couldn't copy ../locales/app.yaml; source path = {}; target path = {}; error = {}",
            source.to_str().unwrap(), 
            target.to_str().unwrap(),
            err);
    }
}