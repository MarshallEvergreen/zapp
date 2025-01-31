use std::process::Command;

use vfs::VfsError;

use crate::python_file_system::{
    directory::PythonDirectory,
    errors::{PfsError, PfsErrorKind},
    interface::{IPythonEntityVisitor, VisitResult},
    source_file::PythonSourceFile,
};

const RUFF: &str = "ruff"; // Change this to the program you want to check

pub struct RuffFormatVisitor {}

impl IPythonEntityVisitor for RuffFormatVisitor {
    fn visit_python_directory(&mut self, visitable: &PythonDirectory) -> VisitResult {
        if visitable.filepath().is_root() {
            tracing::info!("Running ruff format on root directory");

            let format = Command::new(RUFF)
                .arg("format")
                .arg(".")
                .output()
                .map_err(|e| VfsError::from(e))?;

            tracing::info!("Running ruff check on root directory");
            let check = Command::new(RUFF)
                .arg("check")
                .arg(".")
                .arg("--select")
                .arg("I")
                .arg("--fix")
                .output()
                .map_err(|e| VfsError::from(e))?;

            match format.status.success() && check.status.success() {
                true => {
                    tracing::info!("Ruff succeeded");
                    Ok(())
                }
                false => Err(PfsError::new(
                    PfsErrorKind::VisitationError("Ruff Failed".to_string()),
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
