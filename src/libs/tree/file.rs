use vfs::VfsPath;

use super::interface::{ApiVisitor, IPythonLayer};
use std::fmt;

pub struct PythonFile {
    pub filepath: VfsPath,
}

impl fmt::Debug for PythonFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PythonFile")
            .field("filepath", &self.filepath)
            .finish()
    }
}

impl PythonFile {
    pub fn new(filepath: VfsPath) -> Self {
        PythonFile { filepath }
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonFile {
    fn run(&self) {
        if let Ok(content) = self.filepath.read_to_string() {
            println!("{}", content);
        } else {
            eprintln!("Failed to read the file: {:?}", self.filepath);
        }
    }

    fn accept(&self, _visitor: &ApiVisitor) {
        todo!()
    }

    fn is_valid(&self) -> bool {
        return true;
    }
}
