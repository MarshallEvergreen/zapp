use std::collections::HashSet;

use super::errors::TreeError;

pub struct ApiVisitor {}

impl ApiVisitor {
    pub fn new() -> Self {
        ApiVisitor {}
    }
}

pub type RunResult = Result<HashSet<String>, TreeError>;

pub trait IPythonLayer {
    fn name(&self) -> String;
    fn run(&self) -> RunResult;
    fn is_valid(&self) -> bool;
    fn accept(&self, visitor: &ApiVisitor);
}
