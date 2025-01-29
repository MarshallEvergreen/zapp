use googletest::prelude::*;
use vfs::{MemoryFS, SeekAndWrite, VfsPath, VfsResult};

pub struct TestVisitingFileTree {
    pub memfs: VfsPath,
}

impl TestVisitingFileTree {
    // Example method that performs some operation on common_data
    pub fn create_file(&self, name: &str) -> VfsResult<Box<dyn SeekAndWrite + Send>> {
        let filepath = self.memfs.join(name)?;
        let _parent = filepath.parent();
        _parent.create_dir_all()?;
        return filepath.create_file();
    }

    pub fn write_to_file(&self, name: &str, content: &str) -> VfsResult<()> {
        let mut writer: Box<dyn vfs::SeekAndWrite + Send> = self.create_file(name)?;
        writer.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn read_file(&self, name: &str) -> VfsResult<String> {
        let contents = self.memfs.join(name)?.read_to_string()?;
        return Ok(contents);
    }
}

impl ConsumableFixture for TestVisitingFileTree {
    fn set_up() -> googletest::Result<Self> {
        return Ok(TestVisitingFileTree {
            memfs: MemoryFS::new().into(),
        });
    }
}
