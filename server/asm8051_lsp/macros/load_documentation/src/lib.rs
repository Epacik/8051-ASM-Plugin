use load_documentation_internal::load_docs;

#[proc_macro]
pub fn load_documentation(_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(load_docs(proc_macro2::TokenStream::from(_stream)))
}
