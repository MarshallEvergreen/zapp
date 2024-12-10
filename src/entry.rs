use vfs::{PhysicalFS, VfsPath, VfsResult};

pub fn abinit(fs: Option<&VfsPath>) -> Result<(), Box<dyn std::error::Error>> {
    let default_fs: VfsPath = PhysicalFS::new("/").into();
    let root = fs.unwrap_or(&default_fs);

    let mut _directories: Vec<VfsPath> = root.walk_dir()?.collect::<VfsResult<Vec<_>>>()?;
    _directories.sort_by_key(|path| path.as_str().to_string());
    let mut directory_strings: Vec<String> = _directories
        .iter() // Iterate over the _directories vector
        .map(|path| path.as_str().to_string()) // Convert each path to a String
        .collect(); // Collect them into a Vec<String>

    Ok(())
}
