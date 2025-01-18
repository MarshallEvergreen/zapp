pub struct ApiVisitor {}

impl ApiVisitor {
    pub fn new() -> Self {
        ApiVisitor {}
    }
}

pub trait IPythonLayer {
    fn run(&self);

    fn is_valid(&self) -> bool;

    fn accept(&self, visitor: &ApiVisitor);
}
