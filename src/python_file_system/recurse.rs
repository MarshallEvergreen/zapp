use std::path::PathBuf;

use tracing::{info, trace, warn};
use vfs::{PhysicalFS, VfsPath};

use crate::python_file_system::errors::PfsErrorKind;

use super::{
    directory::PythonDirectory,
    errors::PfsResult,
    factory::create_python_directory,
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
        warn!("No file system provided, using default.");
        let cwd: PathBuf =
            std::env::current_dir().map_err(|_| PfsErrorKind::FileSystemCreationError)?;
        info!(
            "Using current working directory as root: '{}'",
            cwd.display()
        );
        _default_fs = Box::new(PhysicalFS::new(cwd).into());

        root = _default_fs.as_ref();
    }

    let _root_directory: PythonDirectory = create_python_directory(root)?;

    visitors.iter_mut().for_each(|visitor| {
        // TODO handle the visitation error here.
        _root_directory.accept(visitor.as_mut()).unwrap_or_default();
    });

    Ok(())
}
