use std::fmt;

#[derive(Debug)]
pub enum WorkspaceError {
    InvalidName(String),
    FileUnavailable(String)
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceError::InvalidName(name) => write!(f, "Workspace name is invalid: {}", name),
            WorkspaceError::FileUnavailable(name) => write!(f, "Cannot open/create workspace file: {}", name),
        }
    }
}

#[derive(Debug)]
pub enum TagAddError {
    InvalidName(String),
    InvalidPath(),
}

impl fmt::Display for TagAddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagAddError::InvalidName(name) => write!(f, "Name is invalid: {}", name),
            TagAddError::InvalidPath() => write!(f, "Provided path has no parent"),
        }
    }
}