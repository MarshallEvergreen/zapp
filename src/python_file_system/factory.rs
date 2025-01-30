use tracing::{trace, warn};
use vfs::VfsPath;

use super::{
    directory::PythonDirectory,
    errors::{PfsError, PfsErrorKind, PfsResult},
    interface::IPythonEntity,
    source_file::PythonSourceFile,
};

type OptionalEntity = Option<Box<dyn IPythonEntity>>;

pub(crate) fn entity_factory(path: &VfsPath) -> PfsResult<OptionalEntity> {
    trace!("Building entity at path '{}'", path.as_str());
    if path.is_file()? && path.extension().is_some_and(|e| e == "py") {
        trace!("Entity '{}' will be built as file", path.as_str());
        return Ok(Some(Box::new(PythonSourceFile::new(path.clone()))));
    } else {
        trace!(
            "Attempting to build python directory for entity '{}'",
            path.as_str()
        );
        return Ok(Some(Box::new(create_python_directory(path)?)));
    };
}

pub(crate) fn create_python_directory(path: &VfsPath) -> PfsResult<PythonDirectory> {
    if !path.is_dir()? {
        trace!(
            "Cannot build directory for '{}' - it is a file",
            path.as_str()
        );
        return Err(PfsError::new(
            PfsErrorKind::DirectoryCreationError,
            "Path is not a directory".to_string(),
        ));
    } else if !path.join("__init__.py")?.exists()? {
        warn!(
            "Cannot build directory for '{}' - missing __init__.py",
            path.as_str()
        );
        return Err(PfsError::new(
            PfsErrorKind::DirectoryCreationError,
            "Directory does not contain __init__.py".to_string(),
        ));
    } else {
        trace!("Creating Python directory at path: {}", path.as_str());
        Ok(PythonDirectory::new(path)?)
    }
}
