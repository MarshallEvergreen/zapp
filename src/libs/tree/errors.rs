use vfs::VfsError;

#[derive(Debug)]
pub enum TreeError {
    VfsError(VfsError),
    FileSystemCreationError,
}

impl From<VfsError> for TreeError {
    fn from(err: VfsError) -> Self {
        TreeError::VfsError(err)
    }
}
