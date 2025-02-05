use vfs::VfsPath;

use super::{
    api_file::PythonApiFile,
    errors::PfsResult,
    factory::entity_factory,
    interface::{IPythonEntity, IPythonEntityVisitor, VisitResult},
};

const INIT_PY: &str = "__init__.py";

pub struct PythonDirectory {
    children: Vec<Box<dyn IPythonEntity>>,
    init_file: PythonApiFile,

    name: String,
    filepath: VfsPath,
}

impl PythonDirectory {
    pub fn new(root: &VfsPath) -> PfsResult<PythonDirectory> {
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
            .filter_map(|path: &VfsPath| entity_factory(&path).ok()?)
            .collect();

        Ok(PythonDirectory {
            init_file: PythonApiFile::new(root.join(INIT_PY)?),
            children: _layers,
            name: root.filename().to_string(),
            filepath: root.clone(),
        })
    }

    pub fn filepath(&self) -> &VfsPath {
        &self.filepath
    }

    pub fn init_file(&self) -> &PythonApiFile {
        return &self.init_file;
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

    fn accept(&self, visitor: &mut dyn IPythonEntityVisitor) -> VisitResult {
        for child in &self.children {
            child.accept(visitor)?;
        }
        visitor.visit_python_directory(&self)
        // TODO visit the init file here.
    }
}
