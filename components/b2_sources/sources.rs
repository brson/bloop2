//! Source files and their relationships.
//!
//! Every _module_ is defined by a single source file.
//!
//! Modules are collected into _packages_.
//!
//! Packages live in _namespaces_.
//! The "std" namespace is a privileged namespace for standard packages.
//!
//! Packages form a DAG.
//! Modules form a DAG by default but can be explicitly "friended" to each other.
//!
//! Packages belong to _workspaces_, which may be implicit,
//! and the information within may be flattened into each of
//! the workspace's packages for the purpose of publishing.
//!
//! Packages define dependencies on other packages.
//! Workspaces may control versioning of their packages' dependencies.

use b2_deps::salsa;

use std::path::PathBuf;

#[salsa::jar(db = Db)]
pub struct Jar (
    crate::ModuleSource,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB
where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[salsa::input]
pub struct ModuleSource {
    #[return_ref]
    local_path: PathBuf,
    #[return_ref]
    text: String,
}
