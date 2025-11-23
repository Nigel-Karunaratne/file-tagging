// Files in library
mod workspace;
mod tagfile;
mod errors;
mod tag;

extern crate tempdir; //For unit tests in files

// Public interface for library
pub use workspace::Workspace;
pub use tag::Tag;
pub use errors::WorkspaceError;

