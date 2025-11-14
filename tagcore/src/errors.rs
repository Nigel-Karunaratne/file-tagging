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
pub enum TagFileError {
    BadPath(String),
    Serialize(String),
    Io(std::io::Error)
}

impl fmt::Display for TagFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagFileError::BadPath(value) => write!(f, "Path error: {}", value),
            TagFileError::Serialize(value) => write!(f, "Serialization error: {}", value),
            TagFileError::Io(value) => write!(f, "Encountered IO error: {}", value),
        }
    }
}