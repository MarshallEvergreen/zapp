use vfs::{VfsPath, VfsResult};

use super::interface::{IPythonEntity, IPythonEntityVisitor, VisitResult};

pub struct PythonSourceFile {
    filepath: VfsPath,
}

impl PythonSourceFile {
    pub fn new(filepath: VfsPath) -> Self {
        PythonSourceFile { filepath }
    }

    pub fn filepath(&self) -> &VfsPath {
        return &self.filepath;
    }

    pub fn read_to_string(&self) -> VfsResult<String> {
        return self.filepath().read_to_string();
    }
}

impl IPythonEntity for PythonSourceFile {
    fn name(&self) -> String {
        return self
            .filepath
            .filename()
            .split('.')
            .next()
            .unwrap()
            .to_string();
    }

    fn parent(&self) -> VfsPath {
        self.filepath.parent()
    }

    fn accept(&self, visitor: &mut dyn IPythonEntityVisitor) -> VisitResult {
        visitor.visit_python_source_file(self)
    }
}
