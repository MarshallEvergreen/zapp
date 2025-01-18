use std::path::PathBuf;

use vfs::{PhysicalFS, VfsPath};

use super::{directory::PythonDirectory, errors::TreeError, interface::IPythonLayer};
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

    let mut _python_file_paths: Vec<VfsPath> = root
        .walk_dir()
        .map_err(|_| TreeError::WalkDirectoryError)?
        .filter_map(|entry| {
            let entry: VfsPath = entry.ok()?;
            if entry.is_file().ok()? && entry.extension()? == "py" {
                Some(entry)
            } else {
                None
            }
        })
        .collect();

    _python_file_paths.sort_by_key(|path| path.as_str().to_string());

    // let mut _python_layers: Vec<Box<dyn IPythonLayer>> = _python_file_paths
    //     .iter()
    //     .map(|path| layer_factory(&path))
    //     .collect::<Result<Vec<_>, _>>()?;

    let _root_directory =
        PythonDirectory::new(root).map_err(|_| TreeError::FileSystemCreationError)?;

    _root_directory.run();

    Ok(())
}
