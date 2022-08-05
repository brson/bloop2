pub use b2_deps::salsa;

#[salsa::db(
    b2_token_tree::Jar,
)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {
    fn salsa_runtime(&self) -> &salsa::Runtime {
        self.storage.runtime()
    }
}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(
            Database {
                storage: self.storage.snapshot(),
            }
        )
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            storage: Default::default(),
        }
    }
}
