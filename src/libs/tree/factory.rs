use vfs::VfsPath;

use super::{
    directory::PythonDirectory, errors::TreeError, interface::IPythonLayer,
    source_file::PythonSourceFile,
};

pub fn layer_factory(path: &VfsPath) -> Result<Option<Box<dyn IPythonLayer>>, TreeError> {
    if path.is_file()? && path.extension().is_some_and(|e| e == "py") {
        return Ok(Some(Box::new(PythonSourceFile::new(path.clone()))));
    } else if path.is_dir()? && path.join("__init__.py")?.exists()? {
        return Ok(Some(Box::new(PythonDirectory::new(path)?)));
    }
    Ok(None)
}
