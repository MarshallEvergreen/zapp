use googletest::prelude::*;

use crate::test_helpers::fixtures::TestVisitingFileTree;

#[gtest]
fn test_files_can_be_created(fixture: TestVisitingFileTree) -> googletest::Result<()> {
    // Arrange
    let file_1 = "python_1.py";
    let file_2 = "python_2.py";

    fixture.create_file(file_1);
    fixture.create_file(file_2);

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

    fixture.create_file(file_1);
    fixture.create_file(file_2);

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
