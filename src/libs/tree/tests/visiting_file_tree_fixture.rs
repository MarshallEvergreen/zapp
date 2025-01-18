use googletest::prelude::*;
use vfs::{MemoryFS, VfsPath, VfsResult};

use crate::libs::tree::directory::PythonDirectory;

pub struct TestVisitingFileTree {
    pub memfs: VfsPath,
}

impl TestVisitingFileTree {
    // Example method that performs some operation on common_data
    pub fn create_file(&self, name: &str) -> VfsResult<()> {
        let filepath = self.memfs.join(name)?;
        let _parent = filepath.parent();
        _parent.create_dir_all()?;
        filepath.create_file()?;
        Ok(())
    }

    pub fn write_to_file(&self, name: &str, content: &str) -> VfsResult<()> {
        let mut writer: Box<dyn vfs::SeekAndWrite + Send> = self.memfs.join(name)?.create_file()?;
        writer.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn read_file(&self, name: &str) -> VfsResult<String> {
        let contents = self.memfs.join(name)?.read_to_string()?;
        return Ok(contents);
    }

    pub fn root_dir(&self) -> PythonDirectory {
        return PythonDirectory::new(&self.memfs).unwrap();
    }
}

impl ConsumableFixture for TestVisitingFileTree {
    fn set_up() -> googletest::Result<Self> {
        return Ok(TestVisitingFileTree {
            memfs: MemoryFS::new().into(),
        });
    }
}

#[gtest]
fn test_files_can_be_created(fixture: TestVisitingFileTree) -> googletest::Result<()> {
    // Arrange
    let file_1 = "python_1.py";
    let file_2 = "python_2.py";

    fixture.create_file(file_1)?;
    fixture.create_file(file_2)?;

    // Assert
    let file_1_exists = fixture.memfs.join(file_1)?.exists()?;
    let file_2_exists = fixture.memfs.join(file_2)?.exists()?;

    verify_that!(file_1_exists, eq(true))?;
    verify_that!(file_2_exists, eq(true))
}

#[gtest]
fn test_directories_can_be_created(fixture: TestVisitingFileTree) -> googletest::Result<()> {
    // Arrange
    let file_1 = "foo/python_1.py";
    let file_2 = "bar/python_2.py";

    fixture.create_file(file_1)?;
    fixture.create_file(file_2)?;

    // Assert
    let foo_dir_exists = fixture.memfs.join("foo")?.exists()?;
    let bar_dir_exists = fixture.memfs.join("bar")?.exists()?;

    let p1_exists = fixture.memfs.join(file_1)?.exists()?;
    let p2_exists = fixture.memfs.join(file_2)?.exists()?;

    verify_that!(foo_dir_exists, eq(true))?;
    verify_that!(bar_dir_exists, eq(true))?;
    verify_that!(p1_exists, eq(true))?;
    verify_that!(p2_exists, eq(true))
}
