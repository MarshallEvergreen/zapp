use vfs::VfsPath;

use super::interface::{ApiVisitor, IPythonLayer, RunResult};
use std::collections::HashSet;

pub struct PythonApiFile {
    pub filepath: VfsPath,
}

impl PythonApiFile {
    pub fn new(filepath: VfsPath) -> Self {
        PythonApiFile { filepath }
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonApiFile {
    fn name(&self) -> String {
        self.filepath.as_str().to_string()
    }

    fn api(&self) -> RunResult {
        let public_api = HashSet::new();

        Ok(public_api)
    }

    fn accept(&self, _visitor: &ApiVisitor) {
        todo!()
    }

    fn is_valid(&self) -> bool {
        return true;
    }
}
