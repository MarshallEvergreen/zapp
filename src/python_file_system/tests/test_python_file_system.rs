use googletest::prelude::*;
use indoc::indoc;

use crate::{
    python_file_system::{
        errors::{PfsError, PfsErrorKind},
        recurse::walk,
    },
    test_helpers::fixtures::TestVisitingFileTree,
};

#[gtest]
fn error_if_top_level_directory_missing_init_file(fixture: TestVisitingFileTree) {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def hello_world():
            print("Hello World!")
    "#};

    fixture.write_to_file(file_1, python_hello_world);

    // Act
    let result = walk(vec![], Some(&fixture.memfs));

    let expected_error = PfsError::new(
        PfsErrorKind::DirectoryCreationError,
        "Directory does not contain __init__.py".into(),
    );

    // Assert
    expect_eq!(result.is_err(), true);
    expect_eq!(result, Err(expected_error));
}

#[gtest]
fn error_if_passed_root_is_not_a_directory(fixture: TestVisitingFileTree) {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def hello_world():
            print("Hello World!")
    "#};

    fixture.write_to_file(file_1, python_hello_world);

    // Act
    let result = walk(vec![], Some(&fixture.memfs.join(file_1).unwrap()));

    let expected_error = PfsError::new(
        PfsErrorKind::DirectoryCreationError,
        "Path is not a directory".into(),
    );

    // Assert
    expect_eq!(result.is_err(), true);
    expect_eq!(result, Err(expected_error));
}
