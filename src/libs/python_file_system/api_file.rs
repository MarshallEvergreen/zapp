use std::collections::{BTreeMap, HashSet};

use vfs::VfsPath;

use super::errors::TreeError;

pub struct PythonApiFile {
    pub filepath: VfsPath,
}

impl PythonApiFile {
    pub fn new(filepath: VfsPath) -> Self {
        PythonApiFile { filepath }
    }

    pub fn write(&self, api: &BTreeMap<String, HashSet<String>>) -> Result<(), TreeError> {
        let mut content = String::new();

        for (key, values) in api {
            let values_str = values
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ");

            content.push_str(&format!(
                "from .{} import ({})\n",
                key.replace("/", "."),
                values_str
            ));
        }

        self.filepath
            .create_file()?
            .write_all(content.as_bytes())
            .map_err(|_| TreeError::FileSystemCreationError)?;

        Ok(())
    }
}
