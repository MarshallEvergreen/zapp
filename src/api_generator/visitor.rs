use std::collections::{BTreeMap, HashSet};

use regex::Regex;
use tracing::trace;

use crate::python_file_system::{
    directory::PythonDirectory,
    errors::{PfsError, PfsErrorKind, PfsResult},
    interface::{IPythonEntity, IPythonEntityVisitor, VisitResult},
    source_file::PythonSourceFile,
};

fn python_file_public_api(file: &PythonSourceFile) -> PfsResult<HashSet<String>> {
    let mut public_api = HashSet::new();

    let contents = file.read_to_string()?;

    let all_re = Regex::new(r"__all__\s*=\s*\[(.*?)\]").unwrap();
    let all_re_multiline = Regex::new(r"__all__\s*=\s*\[(?s)(.*?)\]").unwrap();

    let maybe_all = all_re
        .captures(&contents)
        .or_else(|| all_re_multiline.captures(&contents));

    if let Some(all) = maybe_all {
        if let Some(matched) = all.get(1) {
            public_api = matched
                .as_str()
                .split(',')
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\n' || c == ' '))
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
        }
    } else {
        trace!("__all__ not found for {}", file.name());
    }

    trace!("Public API for {}: {:?}", file.name(), public_api);

    Ok(public_api)
}

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
                PfsError::new(
                    PfsErrorKind::VisitationError(format!(
                        "ApiGeneratorVisitor failed to find key {}",
                        visitable.name()
                    )),
                    "Failed to find expected submodule key".to_string(),
                )
            })?;

        let public_api: HashSet<String> = submodule_apis.values().cloned().flatten().collect();
        tracing::info!("Public API for {}: {:?}", visitable.name(), public_api);
        visitable.init_file.write(&submodule_apis)?;

        self.insert_submodule_api(visitable, public_api);
        Ok(())
    }

    fn visit_python_source_file(&mut self, visitable: &PythonSourceFile) -> VisitResult {
        let api = python_file_public_api(visitable)?;
        self.insert_submodule_api(visitable, api);
        Ok(())
    }
}
