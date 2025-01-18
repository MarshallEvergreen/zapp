use vfs::VfsPath;

use crate::libs::tree::directory::PythonDirectory;

use super::{errors::TreeError, file::PythonFile, interface::IPythonLayer};

pub fn layer_factory(path: &VfsPath) -> Result<Option<Box<dyn IPythonLayer>>, TreeError> {
    if path.is_file()? && path.extension().is_some_and(|e| e == "py") {
        return Ok(Some(Box::new(PythonFile::new(path.clone()))));
    } else if path.is_dir()? {
        return Ok(Some(Box::new(PythonDirectory::new(path)?)));
    }
    Ok(None)
}
