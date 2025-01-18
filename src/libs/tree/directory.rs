use super::interface::{ApiVisitor, IPythonLayer};

pub struct PythonDirectory {}

impl PythonDirectory {
    pub fn new() -> Self {
        PythonDirectory {}
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonDirectory {
    fn run(&self) {
        todo!()
    }

    fn accept(&self, _visitor: &ApiVisitor) {
        todo!()
    }
}
