use b2_deps::salsa;

#[salsa::jar(db = Db)]
pub struct Jar {
}

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB
where DB: ?Sized + salsa::DbWithJar<Jar> {}
