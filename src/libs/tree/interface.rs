pub struct ApiVisitor {}

impl ApiVisitor {
    pub fn new() -> Self {
        ApiVisitor {}
    }
}

pub trait IPythonLayer {
    fn run(&self);
    fn accept(&self, visitor: &ApiVisitor);
}
