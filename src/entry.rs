use vfs::{PhysicalFS, VfsPath};

pub fn abinit(fs: Option<&VfsPath>) {
    let default_fs: VfsPath = PhysicalFS::new("/").into();
    let _fs = fs.unwrap_or(&default_fs);
}
