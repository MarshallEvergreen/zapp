use std::path::PathBuf;

use vfs::{PhysicalFS, VfsPath};

use super::{directory::PythonDirectory, errors::TreeError, interface::IPythonLayer};
use std::collections::HashMap;
pub fn walk(fs: Option<&VfsPath>) -> Result<(), TreeError> {
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

    let mut _python_file_paths: HashMap<String, VfsPath> = root
        .walk_dir()?
        .filter_map(|entry| {
            let entry: VfsPath = entry.ok()?;
            if entry.is_file().ok()? && entry.extension().is_some_and(|e| e == "py") {
                let parent = entry.parent();
                Some((parent.as_str().to_string(), parent))
            } else {
                None
            }
        })
        .collect();

    let _root_directory: Box<dyn IPythonLayer> = Box::new(PythonDirectory::new(root)?);

    _root_directory.run();

    Ok(())
}
