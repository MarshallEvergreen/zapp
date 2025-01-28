use std::error;

use vfs::VfsError;

#[derive(Debug)]
pub enum PythonFileSystemError {
    VfsError(VfsError),
    FileSystemCreationError,
    RootDirectoryCreationError(String),
    DirectoryWithoutInitError(String),
}

impl std::fmt::Display for PythonFileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PythonFileSystemError::VfsError(err) => write!(f, "VFS Error: {}", err),
            PythonFileSystemError::FileSystemCreationError => {
                write!(f, "File system creation error")
            }
            PythonFileSystemError::RootDirectoryCreationError(msg) => {
                write!(f, "Root directory creation error: {}", msg)
            }
            PythonFileSystemError::DirectoryWithoutInitError(msg) => {
                write!(f, "Directory without init error: {}", msg)
            }
        }
    }
}

pub type PythonFileSystemResult<T> = std::result::Result<T, PythonFileSystemError>;

impl From<VfsError> for PythonFileSystemError {
    fn from(err: VfsError) -> Self {
        PythonFileSystemError::VfsError(err)
    }
}

impl error::Error for PythonFileSystemError {
    // source() is a method on the Error trait that returns the underlying cause of an error, if it is known.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            PythonFileSystemError::VfsError(err) => Some(err),
            _ => None,
        }
    }
}
