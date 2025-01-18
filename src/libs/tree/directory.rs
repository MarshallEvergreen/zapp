use std::collections::{HashMap, HashSet};

use vfs::VfsPath;

use super::{
    api_file::PythonApiFile,
    errors::TreeResult,
    factory::layer_factory,
    interface::{ApiVisitor, IPythonLayer, RunResult},
};

const INIT_PY: &str = "__init__.py";

pub struct PythonDirectory {
    init_file: PythonApiFile,
    layers: Vec<Box<dyn IPythonLayer>>,
    name: String,
}

impl PythonDirectory {
    pub fn new(root: &VfsPath) -> TreeResult<PythonDirectory> {
        let paths: Vec<VfsPath> = root
            .read_dir()?
            .filter_map(|p| {
                if p.filename().eq(INIT_PY) {
                    None
                } else {
                    Some(p)
                }
            })
            .collect();

        // TODO handle errors better here.
        let _layers: Vec<Box<dyn IPythonLayer>> = paths
            .iter()
            .filter_map(|path: &VfsPath| layer_factory(&path).ok()?)
            .collect();

        Ok(PythonDirectory {
            init_file: PythonApiFile::new(root.join(INIT_PY)?),
            layers: _layers,
            name: root.filename().to_string(),
        })
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonDirectory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn api(&self) -> RunResult {
        let mut submodule_apis: HashMap<String, HashSet<String>> = HashMap::new();

        for layer in &self.layers {
            let api = layer.api()?;
            submodule_apis.insert(layer.name(), api);
        }

        self.init_file.write(&submodule_apis)?;

        let public_api: HashSet<String> = submodule_apis.values().cloned().flatten().collect();

        tracing::info!("Public API for {}: {:?}", self.name(), public_api);

        Ok(public_api)
    }

    fn is_valid(&self) -> bool {
        for layer in &self.layers {
            if layer.is_valid() {
                return true;
            }
        }
        return false;
    }

    fn accept(&self, _visitor: &ApiVisitor) {
        todo!()
    }
}
