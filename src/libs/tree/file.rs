use super::interface::{ApiVisitor, IPythonLayer};

pub struct PythonFile {}

impl PythonFile {
    pub fn new() -> Self {
        PythonFile {}
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonFile {
    fn run(&self) {
        todo!()
    }

    fn accept(&self, visitor: &ApiVisitor) {
        todo!()
    }
}
