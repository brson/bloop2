use b2_deps::salsa;
use b2_token_tree::TokenTree;
use b2_sources::ModuleSource;

#[salsa::jar(db = Db)]
pub struct Jar (
    crate::lex,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB
where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[salsa::tracked(return_ref)]
pub fn lex(db: &dyn Db,
           source: ModuleSource) -> TokenTree {
    #![allow(unused)]
    todo!()
}
