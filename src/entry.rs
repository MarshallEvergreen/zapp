use crate::libs::tree::{errors::TreeError, walk::walk};
use vfs::VfsPath;

pub fn abinit(fs: Option<&VfsPath>) -> Result<(), TreeError> {
    walk(fs)?;
    Ok(())
}
