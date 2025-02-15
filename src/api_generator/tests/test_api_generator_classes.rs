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
fn create_api_public_classes_are_detected(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let contents: &str = indoc! {r#"
        class TopLevelClass:
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, contents);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (TopLevelClass)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_polymorphic_public_classes_are_detected(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let contents: &str = indoc! {r#"
        class Base:
            pass

        class Child(Base):
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, contents);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (Base, Child)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_private_classes_are_ignored(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let contents: &str = indoc! {r#"
        class _Base:
            pass

        class Child(Base):
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, contents);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (Child)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_all_list_takes_preference_over_declared_classes(
    fixture: TestVisitingFileTree,
) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let contents: &str = indoc! {r#"

        __all__ = ["Child"]

        class Base:
            pass

        class Child(Base):
            pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, contents);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (Child)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}

#[gtest]
fn create_api_nested_classes_are_ignored(fixture: TestVisitingFileTree) -> Result<()> {
    // Arrange
    let file_1 = "python_1.py";

    let contents: &str = indoc! {r#"
        class Base:
            pass

            class NestedChild:
                pass
    "#};

    fixture.create_file("__init__.py");
    fixture.write_to_file(file_1, contents);

    // Act
    fixture.walk(api_visitor());
    // Assert

    let expected_contents = indoc! {r#"
        from .python_1 import (Base)
    "#};

    let actual_contents: String = fixture.read_file("__init__.py");

    verify_that!(actual_contents, eq(expected_contents))
}
