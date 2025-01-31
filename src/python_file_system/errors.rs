use std::{error, fmt};

use vfs::VfsError;

#[derive(Debug, PartialEq)]
pub struct PfsError {
    /// The kind of error
    kind: PfsErrorKind,
    /// An optional human-readable string describing the context for this error
    context: String,
}

impl PfsError {
    pub fn new(kind: PfsErrorKind, context: String) -> Self {
        PfsError { kind, context }
    }

    pub fn kind(&self) -> &PfsErrorKind {
        &self.kind
    }
}

pub type PfsResult<T> = std::result::Result<T, PfsError>;

impl From<PfsErrorKind> for PfsError {
    fn from(kind: PfsErrorKind) -> Self {
        PfsError {
            kind,
            context: "An error occurred".into(),
        }
    }
}

impl From<VfsError> for PfsError {
    fn from(err: VfsError) -> Self {
        Self::from(PfsErrorKind::VfsError(err))
    }
}

impl From<regex::Error> for PfsError {
    fn from(err: regex::Error) -> Self {
        PfsError {
            kind: PfsErrorKind::VisitationError(format!("{}", err)),
            context: "Regex Error.".into(),
        }
    }
}

impl fmt::Display for PfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind(), self.context)
    }
}

impl error::Error for PfsError {
    // source() is a method on the Error trait that returns the underlying cause of an error, if it is known.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.kind() {
            PfsErrorKind::VfsError(err) => Some(err),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PfsErrorKind {
    VfsError(VfsError),
    FileSystemCreationError,
    DirectoryCreationError,
    VisitationError(String),
}

impl PartialEq for PfsErrorKind {
    fn eq(&self, other: &Self) -> bool {
        use PfsErrorKind::*;
        match (self, other) {
            (VfsError(_), VfsError(_))
            | (FileSystemCreationError, FileSystemCreationError)
            | (DirectoryCreationError, DirectoryCreationError) => true,
            (VisitationError(a), VisitationError(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for PfsErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PfsErrorKind::VfsError(err) => write!(f, "VFS error: {}", err),
            PfsErrorKind::FileSystemCreationError => {
                write!(f, "File system creation error")
            }
            PfsErrorKind::DirectoryCreationError => {
                write!(f, "Directory without __init__.py error")
            }
            PfsErrorKind::VisitationError(msg) => {
                write!(f, "Python entity visitation error: '{}'", msg)
            }
        }
    }
}
