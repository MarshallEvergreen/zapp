use std::error;

use vfs::VfsError;

#[derive(Debug)]
pub enum TreeError {
    VfsError(VfsError),
    FileSystemCreationError,
    RootDirectoryCreationError(String),
    DirectoryWithoutInitError(String),
}

impl std::fmt::Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeError::VfsError(err) => write!(f, "VFS Error: {}", err),
            TreeError::FileSystemCreationError => write!(f, "File system creation error"),
            TreeError::RootDirectoryCreationError(msg) => {
                write!(f, "Root directory creation error: {}", msg)
            }
            TreeError::DirectoryWithoutInitError(msg) => {
                write!(f, "Directory without init error: {}", msg)
            }
        }
    }
}

pub type TreeResult<T> = std::result::Result<T, TreeError>;

impl From<VfsError> for TreeError {
    fn from(err: VfsError) -> Self {
        TreeError::VfsError(err)
    }
}

impl error::Error for TreeError {
    // source() is a method on the Error trait that returns the underlying cause of an error, if it is known.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            TreeError::VfsError(err) => Some(err),
            _ => None,
        }
    }
}
