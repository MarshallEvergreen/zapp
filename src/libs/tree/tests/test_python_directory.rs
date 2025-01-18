use googletest::prelude::*;

use crate::libs::tree::{directory::PythonDirectory, interface::IPythonLayer};

use super::visiting_file_tree_fixture::TestVisitingFileTree;

#[gtest]
fn test_python_directory_containing_top_level_python_files_is_valid(
    fixture: TestVisitingFileTree,
) -> googletest::Result<()> {
    // Arrange
    let file_1 = "python_1.py";
    let file_2 = "python_2.py";

    fixture.create_file(file_1)?;
    fixture.create_file(file_2)?;

    // Act
    let root: PythonDirectory = fixture.root_dir();

    // Assert

    verify_that!(root.is_valid(), eq(true))
}

#[gtest]
fn test_python_directory_containing_no_top_level_python_files_is_valid(
    fixture: TestVisitingFileTree,
) -> googletest::Result<()> {
    // Arrange
    let file_1 = "file.txt";
    let file_2 = "file.txt";

    fixture.create_file(file_1)?;
    fixture.create_file(file_2)?;

    // Act
    let root: PythonDirectory = fixture.root_dir();

    // Assert

    verify_that!(root.is_valid(), eq(false))
}

#[gtest]
fn test_python_directory_containing_mixture_of_files_is_still_valid(
    fixture: TestVisitingFileTree,
) -> googletest::Result<()> {
    // Arrange
    let file_1 = "file.py";
    let file_2 = "file.txt";

    fixture.create_file(file_1)?;
    fixture.create_file(file_2)?;

    // Act
    let root: PythonDirectory = fixture.root_dir();

    // Assert

    verify_that!(root.is_valid(), eq(true))
}
