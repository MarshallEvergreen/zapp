use std::{error, fmt};

use vfs::VfsError;

#[derive(Debug, PartialEq)]
pub struct PythonFileSystemError {
    /// The kind of error
    kind: PythonFileSystemErrorKind,
    /// An optional human-readable string describing the context for this error
    context: String,
}

impl PythonFileSystemError {
    pub fn new(kind: PythonFileSystemErrorKind, context: String) -> Self {
        PythonFileSystemError { kind, context }
    }

    pub fn kind(&self) -> &PythonFileSystemErrorKind {
        &self.kind
    }
}

pub type PythonFileSystemResult<T> = std::result::Result<T, PythonFileSystemError>;

impl From<PythonFileSystemErrorKind> for PythonFileSystemError {
    fn from(kind: PythonFileSystemErrorKind) -> Self {
        PythonFileSystemError {
            kind,
            context: "An error occurred".into(),
        }
    }
}

impl From<VfsError> for PythonFileSystemError {
    fn from(err: VfsError) -> Self {
        Self::from(PythonFileSystemErrorKind::VfsError(err))
    }
}

impl fmt::Display for PythonFileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind(), self.context)
    }
}

impl error::Error for PythonFileSystemError {
    // source() is a method on the Error trait that returns the underlying cause of an error, if it is known.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.kind() {
            PythonFileSystemErrorKind::VfsError(err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PythonFileSystemErrorKind {
    VfsError(VfsError),
    FileSystemCreationError,
    RootDirectoryCreationError,
    DirectoryWithoutInitError,
    PythonEntityVisitationError(String),
}

impl PartialEq for PythonFileSystemErrorKind {
    fn eq(&self, other: &Self) -> bool {
        use PythonFileSystemErrorKind::*;
        match (self, other) {
            (VfsError(_), VfsError(_))
            | (FileSystemCreationError, FileSystemCreationError)
            | (RootDirectoryCreationError, RootDirectoryCreationError)
            | (DirectoryWithoutInitError, DirectoryWithoutInitError) => true,
            (PythonEntityVisitationError(a), PythonEntityVisitationError(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for PythonFileSystemErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonFileSystemErrorKind::VfsError(err) => write!(f, "VFS error: {}", err),
            PythonFileSystemErrorKind::FileSystemCreationError => {
                write!(f, "File system creation error")
            }
            PythonFileSystemErrorKind::RootDirectoryCreationError => {
                write!(f, "Root directory creation error")
            }
            PythonFileSystemErrorKind::DirectoryWithoutInitError => {
                write!(f, "Directory without __init__.py error")
            }
            PythonFileSystemErrorKind::PythonEntityVisitationError(msg) => {
                write!(f, "Python entity visitation error: '{}'", msg)
            }
        }
    }
}
