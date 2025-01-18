use std::collections::{HashMap, HashSet};

use vfs::VfsPath;

use super::{
    errors::TreeError,
    factory::layer_factory,
    file::PythonFile,
    interface::{ApiVisitor, IPythonLayer, RunResult},
};

pub struct PythonDirectory {
    init_file: PythonFile,
    layers: Vec<Box<dyn IPythonLayer>>,
    pub name: String,
}

impl PythonDirectory {
    pub fn new(root: &VfsPath) -> Result<Self, TreeError> {
        let _init_file = PythonFile::new(root.join("__init__.py")?);

        let mut paths: Vec<VfsPath> = root.read_dir()?.collect();
        paths.sort_by_key(|path| path.as_str().to_string());

        // TODO handle errors better here.
        let _layers: Vec<Box<dyn IPythonLayer>> = paths
            .iter()
            .filter_map(|path: &VfsPath| layer_factory(&path).ok()?)
            .collect();

        Ok(PythonDirectory {
            init_file: PythonFile::new(root.join("__init__.py")?),
            layers: _layers,
            name: root.as_str().to_string(),
        })
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonDirectory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&self) -> RunResult {
        let mut submodule_apis: HashMap<String, HashSet<String>> = HashMap::new();

        for layer in &self.layers {
            let api = layer.run()?;
            submodule_apis.insert(layer.name(), api);
        }

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
