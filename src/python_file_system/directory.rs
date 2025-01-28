use std::collections::{BTreeMap, HashSet};

use vfs::VfsPath;

use super::{
    api_file::PythonApiFile,
    errors::PythonFileSystemResult,
    factory::layer_factory,
    interface::{IPythonEntity, IPythonEntityVisitor, RunResult, VisitResult},
};

const INIT_PY: &str = "__init__.py";

pub struct PythonDirectory {
    pub layers: Vec<Box<dyn IPythonEntity>>,
    pub init_file: PythonApiFile,

    name: String,
    filepath: VfsPath,
}

impl PythonDirectory {
    pub fn new(root: &VfsPath) -> PythonFileSystemResult<PythonDirectory> {
        let _paths: Vec<VfsPath> = root
            .read_dir()?
            .filter_map(|p| {
                if p.filename().eq(INIT_PY) {
                    None
                } else {
                    Some(p)
                }
            })
            .collect();

        let _layers: Vec<Box<dyn IPythonEntity>> = _paths
            .iter()
            .filter_map(|path: &VfsPath| layer_factory(&path).ok()?)
            .collect();

        Ok(PythonDirectory {
            init_file: PythonApiFile::new(root.join(INIT_PY)?),
            layers: _layers,
            name: root.filename().to_string(),
            filepath: root.clone(),
        })
    }
}

// Implement ITask for MyTask
impl IPythonEntity for PythonDirectory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn parent(&self) -> VfsPath {
        self.filepath.parent()
    }

    fn api(&self) -> RunResult {
        let mut submodule_apis: BTreeMap<String, HashSet<String>> = BTreeMap::new();

        for layer in &self.layers {
            let api = layer.api()?;
            submodule_apis.insert(layer.name(), api);
        }

        self.init_file.write(&submodule_apis)?;

        let public_api: HashSet<String> = submodule_apis.values().cloned().flatten().collect();

        tracing::info!("Public API for {}: {:?}", self.name(), public_api);

        Ok(public_api)
    }

    fn accept(&self, visitor: &mut dyn IPythonEntityVisitor) -> VisitResult {
        for layer in &self.layers {
            layer.accept(visitor)?;
        }
        visitor.visit_python_directory(&self)
    }
}
