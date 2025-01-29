use std::path::PathBuf;

use vfs::{PhysicalFS, VfsPath};

use crate::python_file_system::errors::PythonFileSystemError;

use super::{
    errors::PythonFileSystemResult,
    factory::layer_factory,
    interface::{IPythonEntity, IPythonEntityVisitor},
};

pub fn walk(
    mut visitors: Vec<Box<dyn IPythonEntityVisitor>>,
    fs: Option<&VfsPath>,
) -> PythonFileSystemResult<()> {
    let root: &VfsPath;

    // null pointer - only created if no file system is provided
    let _default_fs: Box<VfsPath>;

    if let Some(provided_fs) = fs {
        tracing::info!("File system provided.");
        root = provided_fs;
    } else {
        tracing::warn!("No file system provided, using default.");
        let cwd: PathBuf =
            std::env::current_dir().map_err(|_| PythonFileSystemError::FileSystemCreationError)?;
        tracing::info!(
            "Using current working directory as root: '{}'",
            cwd.display()
        );
        _default_fs = Box::new(PhysicalFS::new(cwd).into());
        root = _default_fs.as_ref();
    }

    let _root_directory: Box<dyn IPythonEntity> = layer_factory(root)?.ok_or_else(|| {
        PythonFileSystemError::RootDirectoryCreationError(format!(
            "Failed to create root directory layer",
        ))
    })?;

    visitors.iter_mut().for_each(|visitor| {
        // TODO handle errors
        _root_directory.accept(visitor.as_mut()).unwrap_or_default();
    });

    Ok(())
}
