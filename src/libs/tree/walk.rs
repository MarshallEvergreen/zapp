use vfs::{PhysicalFS, VfsPath, VfsResult};

use super::{errors::TreeError, factory::layer_factory, interface::IPythonLayer};

pub fn walk(fs: Option<&VfsPath>) -> Result<(), TreeError> {
    let default_fs: VfsPath = PhysicalFS::new("/").into();
    let root = fs.unwrap_or(&default_fs);

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
