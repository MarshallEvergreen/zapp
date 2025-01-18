use crate::libs::tree::{errors::TreeResult, walk::walk};

use super::visiting_file_tree_fixture::TestVisitingFileTree;
use googletest::prelude::*;
use indoc::indoc;

#[gtest]
fn error_if_top_level_directory_missing_init_file(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def hello_world():
            print("Hello World!")
    "#};

    fixture.write_to_file(file_1, python_hello_world)?;

    // Act
    let result: TreeResult<()> = walk(Some(&fixture.memfs));

    // Assert
    // TODO Partial equal needs defining the check errors match
    verify_that!(result.is_err(), eq(true))
}

#[gtest]
fn create_api_created_if_root_directory_is_valid(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        __all__ = ["hello_world"]
        
        def hello_world():
            print("Hello World!")
    "#};

    fixture.create_file("__init__.py")?;
    fixture.write_to_file(file_1, python_hello_world)?;

    // Act
    walk(Some(&fixture.memfs))?;

    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import hello_world"
    "#};

    let actual_contents: String = fixture.read_file("__init__.py")?;

    verify_that!(actual_contents, eq(expected_contents))
}
