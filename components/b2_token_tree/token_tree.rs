use b2_deps::salsa;

#[salsa::jar(db = Db)]
pub struct Jar (
    crate::TokenTree,
    crate::Word,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB
where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[salsa::tracked]
pub struct TokenTree {
    #[return_ref]
    pub tokens: Vec<TokenOrTree>
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum TokenOrTree {
    Token(Token),
    Tree(Tree),
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Tree {
    pub kind: TreeKind,
    pub tree: TokenTree,
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum TreeKind {
    Paren,
    Brace,
    Square,
    Angle,
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Token {
    Ident(Word),
    Number(Word),
    Punctuation(Punctuation),
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Punctuation {
    Colon,
    Dot,
    Comma,
    Percent,
}

#[salsa::interned]
pub struct Word {
    string: String,
}
