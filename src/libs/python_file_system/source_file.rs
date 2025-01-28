use vfs::VfsPath;

use super::interface::{IPythonEntity, IPythonEntityVisitor, RunResult, VisitResult};
use regex::Regex;
use std::collections::HashSet;

pub struct PythonSourceFile {
    filepath: VfsPath,
}

impl PythonSourceFile {
    pub fn new(filepath: VfsPath) -> Self {
        PythonSourceFile { filepath }
    }
}

impl IPythonEntity for PythonSourceFile {
    fn name(&self) -> String {
        // Whether or not to do relative imports can be controlled here.

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

    fn api(&self) -> RunResult {
        let contents = self.filepath.read_to_string()?;
        let re = Regex::new(r"__all__\s*=\s*\[(.*?)\]").unwrap();
        let re_multiline = Regex::new(r"__all__\s*=\s*\[(?s)(.*?)\]").unwrap();

        let mut public_api = HashSet::new();

        if let Some(captures) = re
            .captures(&contents)
            .or_else(|| re_multiline.captures(&contents))
        {
            // Assumes there is only one match
            // TODO raise an exception if there are multiple matches
            if let Some(matched) = captures.get(1) {
                public_api = matched
                    .as_str()
                    .split(',')
                    .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\n' || c == ' '))
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
            }
        }

        tracing::info!("Public API for {}: {:?}", self.name(), public_api);

        Ok(public_api)
    }

    fn accept(&self, visitor: &mut dyn IPythonEntityVisitor) -> VisitResult {
        visitor.visit_python_source_file(self)
    }
}
