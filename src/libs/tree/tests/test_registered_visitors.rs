use crate::libs::tree::tests::visiting_file_tree_fixture::*;
use crate::libs::tree::recurse::walk;
use googletest::prelude::*;

#[gtest]
fn test_files_can_be_created(fixture: TestVisitingFileTree) -> googletest::Result<()> {
    // Arrange
    let file_1 = "python_1.py";
    let file_2 = "python_2.py";

    fixture.create_file(file_1)?;
    fixture.create_file(file_2)?;

    // Act
    let _ = walk(Some(&fixture.memfs));
    // Assert
    let file_1_exists = fixture.memfs.join(file_1)?.exists()?;
    let file_2_exists = fixture.memfs.join(file_2)?.exists()?;

    verify_that!(file_1_exists, eq(true))?;
    verify_that!(file_2_exists, eq(true))
}
