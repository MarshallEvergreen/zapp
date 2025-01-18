use vfs::VfsPath;

use super::interface::{ApiVisitor, IPythonLayer};

pub struct PythonFile {
    pub filepath: VfsPath,
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
}
