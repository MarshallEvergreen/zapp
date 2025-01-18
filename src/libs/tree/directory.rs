use vfs::VfsPath;

use super::{
    factory::layer_factory,
    file::PythonFile,
    interface::{ApiVisitor, IPythonLayer},
};

pub struct PythonDirectory {
    init_file: PythonFile,
    layers: Vec<Box<dyn IPythonLayer>>,
}

impl PythonDirectory {
    pub fn new(root: &VfsPath) -> Result<Self, vfs::VfsError> {
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
        })
    }
}

// Implement ITask for MyTask
impl IPythonLayer for PythonDirectory {
    fn run(&self) {
        for layer in &self.layers {
            layer.run();
        }
    }

    fn accept(&self, _visitor: &ApiVisitor) {
        todo!()
    }
}
