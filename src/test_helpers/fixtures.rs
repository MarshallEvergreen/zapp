use googletest::prelude::*;
use vfs::{MemoryFS, SeekAndWrite, VfsPath};

use crate::python_file_system::{
    errors::PfsResult, interface::IPythonEntityVisitor, recurse::walk,
};

pub struct TestVisitingFileTree {
    pub memfs: VfsPath,
}

impl TestVisitingFileTree {
    // Example method that performs some operation on common_data
    pub fn create_file(&self, name: &str) -> Box<dyn SeekAndWrite + Send> {
        let filepath = self.memfs.join(name).unwrap();
        let _parent = filepath.parent();
        _parent.create_dir_all().unwrap();
        return filepath.create_file().unwrap();
    }

    pub fn write_to_file(&self, name: &str, content: &str) {
        let mut writer: Box<dyn vfs::SeekAndWrite + Send> = self.create_file(name);
        writer.write_all(content.as_bytes()).unwrap();
    }

    pub fn read_file(&self, name: &str) -> String {
        let contents = self.memfs.join(name).unwrap().read_to_string().unwrap();
        return contents;
    }

    pub fn walk(&self, visitors: Vec<Box<dyn IPythonEntityVisitor>>) {
        walk(visitors, Some(&self.memfs)).unwrap();
    }

    pub fn walk_with_result(&self, visitors: Vec<Box<dyn IPythonEntityVisitor>>) -> PfsResult<()> {
        walk(visitors, Some(&self.memfs))?;
        Ok(())
    }
}

impl ConsumableFixture for TestVisitingFileTree {
    fn set_up() -> googletest::Result<Self> {
        return Ok(TestVisitingFileTree {
            memfs: MemoryFS::new().into(),
        });
    }
}
