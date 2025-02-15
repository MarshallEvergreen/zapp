use std::collections::{BTreeMap, BTreeSet};

use regex::Regex;
use tracing::trace;

use crate::python_file_system::{
    directory::PythonDirectory,
    errors::{PfsError, PfsErrorKind, PfsResult},
    interface::{IPythonEntity, IPythonEntityVisitor, VisitResult},
    source_file::PythonSourceFile,
};

fn public_api_from_all_list(
    public_api: &mut BTreeSet<String>,
    all: regex::Captures<'_>,
) -> PfsResult<()> {
    if let Some(matched) = all.get(1) {
        matched
            .as_str()
            .split(',')
            .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\n' || c == ' '))
            .filter(|s| !s.is_empty())
            .for_each(|s| {
                public_api.insert(s.to_string());
            });
    }

    Ok(())
}

fn python_file_public_api(file: &PythonSourceFile) -> PfsResult<BTreeSet<String>> {
    let mut public_api = BTreeSet::new();

    let contents = file.read_to_string()?;

    let all_re = Regex::new(r"__all__\s*=\s*\[(.*?)\]")?;
    let all_re_multiline = Regex::new(r"__all__\s*=\s*\[(?s)(.*?)\]")?;

    let maybe_all = all_re
        .captures(&contents)
        .or_else(|| all_re_multiline.captures(&contents));

    if let Some(all) = maybe_all {
        trace!(
            "__all__ found for '{}' - This will be used to define the public api",
            file.name()
        );
        public_api_from_all_list(&mut public_api, all)?;
    } else {
        trace!(
            "__all__ not found for '{}' public api will be built from public objects",
            file.name()
        );

        let functions = Regex::new(r"(?m)^def (\w+)\(")?;
        let classes = Regex::new(r"(?m)^class (\w+)\s*(?:\(|:)")?;
        let assignments = Regex::new(r"(?m)^(\w+)\s*=")?;

        let expressions = vec![&functions, &classes, &assignments];

        expressions
            .iter()
            .flat_map(|r| r.captures_iter(&contents))
            .filter_map(|cap| cap.get(1))
            .map(|match_| match_.as_str().to_string())
            .for_each(|name| match name.strip_prefix('_') {
                Some(_) => trace!("Omitted '{}' as it is assumed to be private", name),
                None => {
                    public_api.insert(name.to_string());
                }
            });
    }
    trace!("Public API for {}: {:?}", file.name(), public_api);

    Ok(public_api)
}

pub struct ApiGeneratorVisitor {
    submodule_apis: BTreeMap<String, BTreeMap<String, BTreeSet<String>>>,
}

impl ApiGeneratorVisitor {
    pub fn new() -> Self {
        ApiGeneratorVisitor {
            submodule_apis: BTreeMap::new(),
        }
    }

    fn insert_submodule_api(&mut self, visitable: &dyn IPythonEntity, api: BTreeSet<String>) {
        let parent_key = visitable.parent().filename().as_str().to_string();

        if !api.is_empty() {
            self.submodule_apis
                .entry(parent_key)
                .or_insert_with(BTreeMap::new)
                .insert(visitable.name(), api);
        }
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

        let public_api: BTreeSet<String> = submodule_apis.values().cloned().flatten().collect();
        tracing::info!("Public API for {}: {:?}", visitable.name(), public_api);
        visitable.init_file().write(&submodule_apis)?;

        self.insert_submodule_api(visitable, public_api);
        Ok(())
    }

    fn visit_python_source_file(&mut self, visitable: &PythonSourceFile) -> VisitResult {
        let api = python_file_public_api(visitable)?;
        self.insert_submodule_api(visitable, api);
        Ok(())
    }
}
