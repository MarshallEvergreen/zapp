use std::collections::{BTreeMap, HashSet};

use crate::python_file_system::{
    directory::PythonDirectory,
    errors::PythonFileSystemError,
    interface::{IPythonEntity, IPythonEntityVisitor, VisitResult},
    source_file::PythonSourceFile,
};

pub struct ApiGeneratorVisitor {
    submodule_apis: BTreeMap<String, BTreeMap<String, HashSet<String>>>,
}

impl ApiGeneratorVisitor {
    pub fn new() -> Self {
        ApiGeneratorVisitor {
            submodule_apis: BTreeMap::new(),
        }
    }

    fn insert_submodule_api(&mut self, visitable: &dyn IPythonEntity, api: HashSet<String>) {
        let parent_key = visitable.parent().filename().as_str().to_string();
        self.submodule_apis
            .entry(parent_key)
            .or_insert_with(BTreeMap::new)
            .insert(visitable.name(), api);
    }
}

impl IPythonEntityVisitor for ApiGeneratorVisitor {
    fn visit_python_directory(&mut self, visitable: &PythonDirectory) -> VisitResult {
        let submodule_apis = self
            .submodule_apis
            .get_mut(&visitable.name())
            .ok_or_else(|| {
                tracing::error!("Failed to find key {}", visitable.name());
                // TODO better error here
                PythonFileSystemError::RootDirectoryCreationError(format!(
                    "Failed to find key {}",
                    visitable.name()
                ))
            })?;

        let public_api: HashSet<String> = submodule_apis.values().cloned().flatten().collect();
        tracing::info!("Public API for {}: {:?}", visitable.name(), public_api);
        visitable.init_file.write(&submodule_apis)?;

        self.insert_submodule_api(visitable, public_api);
        Ok(())
    }

    fn visit_python_source_file(&mut self, visitable: &PythonSourceFile) -> VisitResult {
        let api = visitable.api()?;
        self.insert_submodule_api(visitable, api);
        Ok(())
    }
}
