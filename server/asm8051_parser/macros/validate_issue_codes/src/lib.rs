use proc_macro::TokenStream as TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{File, visit::Visit};
use validate_issue_codes_internal::Visitor;

#[proc_macro]
pub fn validate_issuecode_uniqueness(item: TokenStream) -> TokenStream {
    validate_issuecode_uniqueness1(TokenStream2::from(item.clone()));
    item
}

fn validate_issuecode_uniqueness1(code: TokenStream2) {
    let syntax_tree: File = syn::parse2(code).unwrap();
        let mut visitor = Visitor::new();
        visitor.visit_file(&syntax_tree);
}
