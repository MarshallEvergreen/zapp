use googletest::prelude::*;
use indoc::indoc;

use crate::{
    python_file_system::{
        errors::{PythonFileSystemError, PythonFileSystemErrorKind},
        recurse::walk,
    },
    test_helpers::fixtures::TestVisitingFileTree,
    ApiGeneratorVisitor,
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
    let result = walk(
        vec![Box::new(ApiGeneratorVisitor::new())],
        Some(&fixture.memfs),
    );

    let expected_error = PythonFileSystemError::new(
        PythonFileSystemErrorKind::RootDirectoryCreationError,
        "Failed to created root directory".into(),
    );

    // Assert
    expect_eq!(result.is_err(), true);
    expect_eq!(result, Err(expected_error));
}
