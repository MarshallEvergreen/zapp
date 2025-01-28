use std::collections::HashSet;

use vfs::VfsPath;

use super::{
    directory::PythonDirectory, errors::PythonFileSystemError, source_file::PythonSourceFile,
};

pub type RunResult = Result<HashSet<String>, PythonFileSystemError>;
pub type VisitResult = Result<(), PythonFileSystemError>;

pub trait IPythonEntity {
    fn name(&self) -> String;
    fn parent(&self) -> VfsPath;

    fn api(&self) -> RunResult;
    fn accept(&self, visitor: &mut dyn IPythonEntityVisitor) -> VisitResult;
}

pub trait IPythonEntityVisitor {
    fn visit_python_directory(&mut self, visitable: &PythonDirectory) -> VisitResult;
    fn visit_python_source_file(&mut self, visitable: &PythonSourceFile) -> VisitResult;
}
