use vfs::VfsPath;

use super::{
    directory::PythonDirectory,
    errors::{PfsError, PfsErrorKind, PfsResult},
    interface::IPythonEntity,
    source_file::PythonSourceFile,
};

type OptionalEntity = Option<Box<dyn IPythonEntity>>;

pub fn layer_factory(path: &VfsPath) -> PfsResult<OptionalEntity> {
    if path.is_file()? && path.extension().is_some_and(|e| e == "py") {
        return Ok(Some(Box::new(PythonSourceFile::new(path.clone()))));
    } else {
        return Ok(Some(Box::new(create_python_directory(path)?)));
    };
}

pub(crate) fn create_python_directory(path: &VfsPath) -> PfsResult<PythonDirectory> {
    if !path.is_dir()? {
        return Err(PfsError::new(
            PfsErrorKind::DirectoryCreationError,
            "Path is not a directory".to_string(),
        ));
    } else if !path.join("__init__.py")?.exists()? {
        return Err(PfsError::new(
            PfsErrorKind::DirectoryCreationError,
            "Directory does not contain __init__.py".to_string(),
        ));
    } else {
        Ok(PythonDirectory::new(path)?)
    }
}
