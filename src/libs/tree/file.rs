use super::interface::{ApiVisitor, IPythonLayer};

pub struct PythonFile {
    pub filepath: String,
}

impl PythonFile {
    pub fn new(filepath: String) -> Self {
        PythonFile { filepath }
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonFile {
    fn run(&self) {
        todo!()
    }

    fn accept(&self, _visitor: &ApiVisitor) {
        todo!()
    }
}
