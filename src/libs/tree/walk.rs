use std::path::PathBuf;

use vfs::{PhysicalFS, VfsPath, VfsResult};

use super::{errors::TreeError, factory::layer_factory, interface::IPythonLayer};

pub fn walk(fs: Option<VfsPath>) -> Result<(), TreeError> {
    let root: VfsPath;
    let cwd: PathBuf = std::env::current_dir().map_err(|_| TreeError::FileSystemCreationError)?;

    if fs.is_some() {
        tracing::info!("File system provided.");
        root = fs.unwrap();
    } else {
        tracing::warn!("No file system provided, using default.");
        tracing::info!(
            "Using current working directory as root: '{}'",
            cwd.display()
        );
        root = PhysicalFS::new(cwd).into();
    }

    tracing::info!("Root file system path: {}", root.as_str());

    let mut _filepaths: Vec<VfsPath> = root
        .walk_dir()
        .map_err(|_| TreeError::WalkDirectoryError)?
        .collect::<VfsResult<Vec<_>>>()
        .map_err(|_| TreeError::WalkDirectoryError)?;

    _filepaths.sort_by_key(|path| path.as_str().to_string());

    let mut _python_layers: Vec<Box<dyn IPythonLayer>> = _filepaths
        .iter()
        .map(|path| layer_factory(&path))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}
