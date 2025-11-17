// Files in library
mod workspace;
mod tagfile;
mod errors;
mod tag;

extern crate tempdir;

// Public interface for library
pub use workspace::Workspace;
pub use errors::WorkspaceError;

