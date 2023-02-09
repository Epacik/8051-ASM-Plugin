use syn::visit::{self, Visit};
use syn::ExprPath;

pub struct Visitor {
    pub issue_identifiers: Vec::<String>,
}
impl Visitor {
    pub fn new() -> Visitor {
        Visitor {
            issue_identifiers: Vec::new(),
        }
    }

    fn validate_id(&mut self, call: syn::ExprCall, path: ExprPath) {

        let seg = match path.path.segments.last() {
            Some(s) => s.clone(),
            None => return,
        };

        let ident = seg.ident.clone().to_string();

        if ident != String::from("issue_info") {
            return;
        }

        // we need to get an ID
        let first_arg = match call.args.first() {
            Some(a) => a.clone(),
            None => return,
        };

        let id = match first_arg {
            syn::Expr::Lit(literal) => literal,
            _ => return,
        };

        let int = match id.lit {
            syn::Lit::Int(i) => i,
            _ => return,
        };

        let digits = String::from(int.base10_digits());

        if self.issue_identifiers.contains(&digits) {
            panic!("Duplicated issue identifier '{}'", digits);
        }

        self.issue_identifiers.push(digits);
    }

    // fn validate_macro(&mut self, node: syn::ExprMacro) {
    //     let mac = node.mac;
    //     let segment = match mac.path.segments.last() {
    //         Some(id) => id.clone(),
    //         None => return,
    //     };

    //     let ident = segment.ident.to_string();

    //     if ident != String::from("new_issue") {
    //         return;
    //     }

    //     let tokens = /*mac.tokens.clone();*/ mac.to_token_stream();

    //     println!("tokens (validate_macro):\n{}", tokens.clone());

    //     for tok in tokens.clone() {
    //         println!("token tree (validate_macro): {}", tok);
    //     }

    //     let syntax_tree = match syn::parse2::<Macro>(tokens) {
    //         Ok(data) => data.clone(),
    //         Err(err) => {
    //             let err = err;
    //             println!("err: {}", err);
    //             return;
    //         },
    //     }; 
    //     let mut visitor = Visitor::new();
    //     visitor.visit_macro(&syntax_tree);

    //     for id in visitor.issue_identifiers {
    //         if self.issue_identifiers.contains(&id){
    //             panic!("Duplicated issue identifier '{}'", id);
    //         }
    //         self.issue_identifiers.push(id);
    //     }
        
    // }

}


impl<'ast> Visit<'ast> for Visitor {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        

        let func = *(node.func.clone());
        match func {
            syn::Expr::Path(x) =>  self.validate_id(node.clone(), x),
            _ => { }
        }
        
        visit::visit_expr_call(self, node);
    }

    // fn visit_expr_macro(&mut self, node: &'ast syn::ExprMacro) {
    //     self.validate_macro(node.clone());

    //     visit::visit_expr_macro(self, node)
    // }
    // fn visit_span(&mut self, node: &proc_macro2::Span) {
        
    //     visit::visit_span(self, node)
    // }

    // fn visit_macro(&mut self, node: &'ast Macro) {

    //     let segment = node.path.segments.last().unwrap();
    //     let ident = segment.ident.to_string();

    //     if ident == String::from("new_issue") {
    //         println!("tokens (visit_macro):\n{}", node.to_token_stream());

    //         for tok in node.to_token_stream() {
    //             println!("token tree (visit_macro): {}", tok);
    //         }
    //     }
        
        
    //     visit::visit_macro(self, node)
    // }

}

#[cfg(test)]
mod tests {
    use crate::Visitor;
    use quote::quote;
    use syn::visit::Visit;
    use syn::File;

    #[test]
    fn test_counting() {
        let code = quote! {
            #[derive(Hash, Eq, PartialEq, Clone)]
            pub struct IssueInfo(u32, IssueType, String);

            #[derive(Hash, Eq, PartialEq, Clone)]
            pub struct Issue {
                span: Position,
                label: IssueInfo,
                expected: Vec<Option<Token>>,
                found: Option<Token>
            }

            fn issue_info(code: u32, default_type: IssueType, message_key: &str) -> IssueInfo {
                IssueInfo(code, default_type, message_key.to_string())
            }

            fn unclosed_delimiter_info() -> IssueInfo {
                issue_info(1001, Error, "unclosed-string")
            }

            fn unclosed_delimiter_info1() -> IssueInfo {
                issue_info(1002, Error, "unclosed-string")
            }

            pub(crate) fn unknown_token(position: Position, found: Token) -> Issue {
                Issue::new(position, issue_info(1001, Error, "unknown-token"), None, Some(found))
            }
        };

        let syntax_tree: File = syn::parse2(code).unwrap();
        let mut visitor = Visitor::new();
        visitor.visit_file(&syntax_tree);

    }

    #[test]
    fn macro_rules_t() {
        let code = quote! {
            #[derive(Hash, Eq, PartialEq, Clone)]
            pub struct IssueInfo(u32, IssueType, String);

            #[derive(Hash, Eq, PartialEq, Clone)]
            pub struct Issue {
                span: Position,
                label: IssueInfo,
                expected: Vec<Option<Token>>,
                found: Option<Token>
            }

            macro_rules! new_issue {
                ($position:expr, $info:expr) => {
                    Issue::new($position, $info, None, None)
                };
                ($position:expr, $info:expr, found: $found:expr) => {
                    Issue::new($position, $info, None, Some($found))
                };
                ($position:expr, $info:expr, expected: $expected:expr) => {
                    Issue::new($position, $info, Some($expected), None)
                };
                ($position:expr, $info:expr, $expected:expr, $found:expr) => {
                    Issue::new($position, $info, Some($expected), Some($found))
                };
            }

            // fn issue_info(code: u32, default_type: IssueType, message_key: &str) -> IssueInfo {
            //     IssueInfo(code, default_type, message_key.to_string())
            // }

            // fn unclosed_delimiter_info() -> IssueInfo {
            //     issue_info(1001, Error, "unclosed-string")
            // }

            // fn unclosed_delimiter_info1() -> IssueInfo {
            //     issue_info(1002, Error, "unclosed-string")
            // }

            pub(crate) fn unknown_token(position: Position, found: Token) -> Issue {
                new_issue!(position, issue_info(1001, Error, "unknown-token"), found: found)
            }

            pub(crate) fn unknown_token1(position: Position, found: Token) -> Issue {
                new_issue!(position, issue_info(1001, Error, "unknown-token"), found: found)
            }
        };

        let syntax_tree: File = syn::parse2(code).unwrap();
        let mut visitor = Visitor::new();
        visitor.visit_file(&syntax_tree);
    }
}