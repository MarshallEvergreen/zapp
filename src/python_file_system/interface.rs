use vfs::VfsPath;

use super::{directory::PythonDirectory, errors::PfsError, source_file::PythonSourceFile};

pub type VisitResult = Result<(), PfsError>;

pub trait IPythonEntity {
    fn name(&self) -> String;
    fn parent(&self) -> VfsPath;
    fn accept(&self, visitor: &mut dyn IPythonEntityVisitor) -> VisitResult;
}

pub trait IPythonEntityVisitor {
    fn visit_python_directory(&mut self, visitable: &PythonDirectory) -> VisitResult;
    fn visit_python_source_file(&mut self, visitable: &PythonSourceFile) -> VisitResult;
}
