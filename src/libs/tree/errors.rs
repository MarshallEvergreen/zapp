use std::fmt;

pub enum TreeError {
    FileSystemCreationError,
    FileCreationError,
    DirectoryCreationError,
    WalkDirectoryError,
}

impl fmt::Debug for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TreeError::FileCreationError => write!(f, "Invalid path type for layer creation"),
            TreeError::DirectoryCreationError => {
                write!(f, "Invalid path type for layer creation")
            }
            TreeError::WalkDirectoryError => write!(f, "Invalid path type for layer creation"),
            TreeError::FileSystemCreationError => write!(f, "Failed to create file system"),
        }
    }
}
