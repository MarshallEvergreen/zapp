use googletest::prelude::*;
use indoc::indoc;

use crate::{
    python_file_system::{errors::PythonFileSystemResult, recurse::walk},
    test_helpers::fixtures::TestVisitingFileTree,
    ApiGeneratorVisitor,
};

#[gtest]
fn error_if_top_level_directory_missing_init_file(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def hello_world():
            print("Hello World!")
    "#};

    fixture.write_to_file(file_1, python_hello_world);

    // Act
    let result: PythonFileSystemResult<()> = walk(
        vec![Box::new(ApiGeneratorVisitor::new())],
        Some(&fixture.memfs),
    );

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

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    walk(
        vec![Box::new(ApiGeneratorVisitor::new())],
        Some(&fixture.memfs),
    )?;
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (hello_world)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_for_multiple_files(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";
    let file_2 = "python_2.py";

    let python_hello_world: &str = indoc! {r#"
        __all__ = ["hello_world"]
        
        def hello_world():
            print("Hello World!")
    "#};

    let python_anti_gravity: &str = indoc! {r#"
    __all__ = ["antigravity"]
    
    def antigravity():
        import antigravity
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);
    fixture.write_to_file(file_2, python_anti_gravity);

    // Act
    walk(
        vec![Box::new(ApiGeneratorVisitor::new())],
        Some(&fixture.memfs),
    )?;
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (hello_world)
        from .python_2 import (antigravity)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_created_if_root_directory_is_valid_for_subdirectory(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "submodule_1/python_1.py";
    let init_file_submodule = "submodule_1/__init__.py";

    let python_hello_world: &str = indoc! {r#"
        __all__ = ["hello_world"]
        
        def hello_world():
            print("Hello World!")
    "#};

    fixture.create_file("__init__.py");
    fixture.create_file(init_file_submodule);
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    walk(
        vec![Box::new(ApiGeneratorVisitor::new())],
        Some(&fixture.memfs),
    )?;

    // Assert

    let expected_top_level_contents = indoc! {r#"
        from .submodule_1 import (hello_world)
    "#};

    let actual_top_level_contents: String = fixture.read_file("__init__.py");

    let expected_submodule_contents = indoc! {r#"
        from .python_1 import (hello_world)
    "#};

    let actual_submodule_contents: String = fixture.read_file("submodule_1/__init__.py");

    verify_that!(actual_top_level_contents, eq(expected_top_level_contents))?;
    verify_that!(actual_submodule_contents, eq(expected_submodule_contents))
}
