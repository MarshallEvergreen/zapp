use std::path::PathBuf;

use tracing::trace;
use vfs::{PhysicalFS, VfsPath};

use crate::python_file_system::errors::PfsErrorKind;

use super::{
    errors::{PfsError, PfsResult},
    factory::layer_factory,
    interface::{IPythonEntity, IPythonEntityVisitor},
};

pub fn walk(
    mut visitors: Vec<Box<dyn IPythonEntityVisitor>>,
    fs: Option<&VfsPath>,
) -> PfsResult<()> {
    let root: &VfsPath;

    // null pointer - only created if no file system is provided
    let _default_fs: Box<VfsPath>;

    if let Some(provided_fs) = fs {
        trace!("File system provided.");
        root = provided_fs;
    } else {
        tracing::warn!("No file system provided, using default.");
        let cwd: PathBuf =
            std::env::current_dir().map_err(|_| PfsErrorKind::FileSystemCreationError)?;
        tracing::info!(
            "Using current working directory as root: '{}'",
            cwd.display()
        );
        _default_fs = Box::new(PhysicalFS::new(cwd).into());

        root = _default_fs.as_ref();
    }

    let _root_directory: Box<dyn IPythonEntity> = layer_factory(root)?.ok_or({
        PfsError::new(
            PfsErrorKind::DirectoryCreationError,
            "Failed to created root directory".into(),
        )
    })?;

    visitors.iter_mut().for_each(|visitor| {
        _root_directory.accept(visitor.as_mut()).unwrap_or_default();
    });

    Ok(())
}
