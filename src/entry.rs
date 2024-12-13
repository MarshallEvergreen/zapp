use vfs::{PhysicalFS, VfsPath, VfsResult};

use crate::libs::tree::interface::IPythonLayer;

pub fn abinit(fs: Option<&VfsPath>) -> Result<(), Box<dyn std::error::Error>> {
    let default_fs: VfsPath = PhysicalFS::new("/").into();
    let root = fs.unwrap_or(&default_fs);

    let mut _filepaths: Vec<VfsPath> = root.walk_dir()?.collect::<VfsResult<Vec<_>>>()?;
    _filepaths.sort_by_key(|path| path.as_str().to_string());

    let mut _directory_strings: Vec<String> = _filepaths
        .iter()
        .map(|path| path.as_str().to_string()) // Convert each path to a String
        .collect();

    let mut python_layers: Vec<Box<dyn IPythonLayer>> =
        _filepaths.iter().map(|path| layer_factory(path)).collect();

    Ok(())
}

fn layer_factory(path: &VfsPath) -> Box<dyn IPythonLayer> {
    todo!()
}
