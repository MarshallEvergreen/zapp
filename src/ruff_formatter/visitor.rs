use std::process::Command;

use vfs::VfsError;

use crate::python_file_system::{
    directory::PythonDirectory,
    errors::{PythonFileSystemError, PythonFileSystemErrorKind},
    interface::{IPythonEntityVisitor, VisitResult},
    source_file::PythonSourceFile,
};

const RUFF: &str = "ruff"; // Change this to the program you want to check

pub struct RuffFormatVisitor {}

impl IPythonEntityVisitor for RuffFormatVisitor {
    fn visit_python_directory(&mut self, visitable: &PythonDirectory) -> VisitResult {
        if visitable.filepath().is_root() {
            tracing::info!(
                "Running ruff on root directory, cmd = {} format {}",
                RUFF,
                visitable.filepath().as_str()
            );
            let output = Command::new(RUFF)
                .arg("format")
                .arg(visitable.filepath().as_str())
                .output()
                .map_err(|e| VfsError::from(e))?
                .status;

            match output.success() {
                true => Ok(()),
                false => Err(PythonFileSystemError::new(
                    PythonFileSystemErrorKind::PythonEntityVisitationError(
                        "Ruff Failed".to_string(),
                    ),
                    "Ruff failed to format the root directory".to_string(),
                )),
            }
        } else {
            Ok(())
        }
    }

    fn visit_python_source_file(&mut self, _visitable: &PythonSourceFile) -> VisitResult {
        Ok(())
    }
}
