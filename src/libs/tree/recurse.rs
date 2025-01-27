use std::path::PathBuf;

use vfs::{PhysicalFS, VfsPath};

use super::{
    errors::{TreeError, TreeResult},
    factory::layer_factory,
    interface::IPythonEntity,
};

pub fn walk(fs: Option<&VfsPath>) -> TreeResult<()> {
    let root: &VfsPath;

    // null pointer - only created if no file system is provided
    let _default_fs: Box<VfsPath>;

    if let Some(provided_fs) = fs {
        tracing::info!("File system provided.");
        root = provided_fs;
    } else {
        tracing::warn!("No file system provided, using default.");
        let cwd: PathBuf =
            std::env::current_dir().map_err(|_| TreeError::FileSystemCreationError)?;
        tracing::info!(
            "Using current working directory as root: '{}'",
            cwd.display()
        );
        _default_fs = Box::new(PhysicalFS::new(cwd).into());
        root = _default_fs.as_ref();
    }

    let _root_directory: Box<dyn IPythonEntity> = layer_factory(root)?.ok_or_else(|| {
        TreeError::RootDirectoryCreationError(format!("Failed to create root directory layer",))
    })?;

    _root_directory.api()?;

    Ok(())
}
