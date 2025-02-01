use std::collections::{BTreeMap, BTreeSet};

use vfs::{VfsError, VfsPath};

use super::errors::PfsError;

pub struct PythonApiFile {
    pub filepath: VfsPath,
}

impl PythonApiFile {
    pub fn new(filepath: VfsPath) -> Self {
        PythonApiFile { filepath }
    }

    pub fn write(&self, api: &BTreeMap<String, BTreeSet<String>>) -> Result<(), PfsError> {
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

        let write_to_file = || -> Result<(), VfsError> {
            self.filepath.create_file()?.write_all(content.as_bytes())?;
            Ok(())
        };

        write_to_file()?;

        Ok(())
    }
}
