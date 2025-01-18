use vfs::VfsError;

#[derive(Debug)]
pub enum TreeError {
    VfsError(VfsError),
    FileSystemCreationError,
    RootDirectoryCreationError(String),
    DirectoryWithoutInitError(String),
}

impl From<VfsError> for TreeError {
    fn from(err: VfsError) -> Self {
        TreeError::VfsError(err)
    }
}
