use crate::issues::Issue;

use self::ast::TopLevel;

mod ast;

pub fn parse<S: AsRef<str>>(_s: S) -> (Option<Vec<TopLevel>>, Vec<Issue>) {

    (None, vec![])
}