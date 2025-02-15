use googletest::prelude::*;
use indoc::indoc;

use crate::{
    api_generator::visitor::ApiGeneratorVisitor,
    python_file_system::interface::IPythonEntityVisitor,
    test_helpers::fixtures::TestVisitingFileTree,
};

fn api_visitor() -> Vec<Box<dyn IPythonEntityVisitor>> {
    let mut visitors: Vec<Box<dyn IPythonEntityVisitor>> = Vec::new();
    visitors.push(Box::new(ApiGeneratorVisitor::new()));
    return visitors;
}

#[gtest]
fn create_api_created_if_all_list_present_in_file(fixture: TestVisitingFileTree) -> Result<()> {
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
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (hello_world)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_created_if_all_list_present_in_file_multiline(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        __all__ = [
            "hello_world",
        ]
        
        def hello_world():
            print("Hello World!")
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (hello_world)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_no_all_and_not_public_functions_results_in_empty_api(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def _hello_world():
            print("Hello World!")
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    fixture.walk(api_visitor());

    // Assert

    let expected_contents = indoc! {r#""#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_all_list_present_overrides_interpreted_public_api(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"

        __all__ = ["top_level"]

        def top_level():
            pass

        def another_top_function():
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (top_level)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_interpreted_from_public_functions_if_all_missing(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def top_level():
            pass

        def another_top_function():
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (another_top_function, top_level)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_interpreted_from_public_functions_and_classes_if_all_missing(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def top_level():
            pass

        class TopLevelClass:
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (TopLevelClass, top_level)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_interpreted_from_public_functions_nested_functions_are_ignored(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let python_hello_world: &str = indoc! {r#"
        def top_level():
            def nested():
                pass
            pass

        def another_top_function():
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, python_hello_world);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (another_top_function, top_level)
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
    fixture.walk(api_visitor());
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
    fixture.walk(api_visitor());

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
