use crate::libs::tree::{errors::TreeResult, walk::walk};

use super::visiting_file_tree_fixture::TestVisitingFileTree;
use googletest::prelude::*;
use indoc::indoc;

#[gtest]
fn test_throws_error_if_top_level_directory_missing_init_file(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def hello_world():
            print("Hello World!")
    "#};

    fixture.write_to_file(file_1, python_hello_world)?;

    // Assert

    let result: TreeResult<()> = walk(Some(&fixture.memfs));

    // TODO Partial equal needs defining the check errors match
    verify_that!(result.is_err(), eq(true))
}

#[gtest]
fn test_walk_create_api(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        __all__ = ["hello_world"]
        
        def hello_world():
            print("Hello World!")
    "#};

    fixture.create_file("__init__.py")?;
    fixture.write_to_file(file_1, python_hello_world)?;

    // Assert

    let result: TreeResult<()> = walk(Some(&fixture.memfs));

    verify_that!(result.is_ok(), eq(true))
}
