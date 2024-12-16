use vfs::VfsPath;

use crate::libs::tree::directory::PythonDirectory;

use super::{errors::TreeError, file::PythonFile, interface::IPythonLayer};

pub fn layer_factory(path: &VfsPath) -> Result<Box<dyn IPythonLayer>, TreeError> {
    if path.is_file().map_err(|_| TreeError::FileCreationError)? {
        return Ok(Box::new(PythonFile {
            filepath: path.as_str().to_string(),
        }));
    }
    if path.is_root()
        || path
            .is_dir()
            .map_err(|_| TreeError::DirectoryCreationError)?
    {
        return Ok(Box::new(PythonDirectory {}));
    }
    // TODO Instead of panicking, return an error for unsupported paths
    panic!()
}
