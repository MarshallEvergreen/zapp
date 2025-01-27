use std::collections::HashSet;

use super::{directory::PythonDirectory, errors::TreeError, source_file::PythonSourceFile};

pub type RunResult = Result<HashSet<String>, TreeError>;

pub trait IPythonEntity {
    fn name(&self) -> String;
    fn api(&self) -> RunResult;
    fn accept(&self, visitor: &mut dyn IPythonEntityVisitor);
}

pub trait IPythonEntityVisitor {
    fn visit_python_directory(&mut self, visitable: &PythonDirectory);
    fn visit_python_source_file(&mut self, visitable: &PythonSourceFile);
}
