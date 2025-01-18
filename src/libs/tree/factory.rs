use vfs::{VfsError, VfsPath};

use crate::libs::tree::directory::PythonDirectory;

use super::{file::PythonFile, interface::IPythonLayer};

pub fn layer_factory(path: &VfsPath) -> Result<Option<Box<dyn IPythonLayer>>, VfsError> {
    if path.is_file()? && path.extension().is_some_and(|e| e == "py") {
        tracing::info!("Building layer for path: {}", path.as_str());
        return Ok(Some(Box::new(PythonFile::new(path.clone()))));
    } else if path.is_dir()? {
        tracing::info!("Building layer for path: {}", path.as_str());
        return Ok(Some(Box::new(PythonDirectory::new(path)?)));
    }
    Ok(None)
}
