use std::fs::File as FsFile;
use std::io::{Error as IoError, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ResourceError {
    Io(IoError),
    /// Invalid when no file extension
    /// Note: File format that Scratch doesn't support is valid
    InvalidFileExtension,
}

impl From<IoError> for ResourceError {
    fn from(value: IoError) -> Self {
        ResourceError::Io(value)
    }
}

impl std::error::Error for ResourceError {}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

/// Preloads the file
/// This might cost some additional memmory but will make the building part uses no result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    extension: String,
    content: Vec<u8>,
    md5_hash: Option<String>,
}

impl Resource {
    /// Does not return IO erorr
    pub fn new(extension: String, content: Vec<u8>) -> Result<Resource, ResourceError> {
        if extension.is_empty() {
            return Err(ResourceError::InvalidFileExtension);
        }
        Ok(Resource {
            extension,
            content,
            md5_hash: None,
        })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Resource, ResourceError> {
        let mut file = FsFile::options().read(true).open(&path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let extension = path
            .as_ref()
            .extension()
            .ok_or(ResourceError::InvalidFileExtension)?
            .to_string_lossy();
        let file = Resource {
            extension: extension.to_string(),
            content: buf,
            md5_hash: None,
        };
        Ok(file)
    }

    /// PathBuf is always valid utf8
    pub fn generate_file_name(&mut self) -> PathBuf {
        let mut path = PathBuf::from(self.get_or_compute_md5_hash());
        path.set_extension(&self.extension);
        path
    }

    pub fn get_or_compute_md5_hash(&mut self) -> &str {
        self.md5_hash.get_or_insert_with(|| {
            md5::compute(&self.content)
                .0
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
    }

    pub fn md5_hash(&self) -> Option<&String> {
        self.md5_hash.as_ref()
    }

    pub fn extension(&self) -> &str {
        &self.extension
    }

    /// Return false if extension is invalid, true otherwise
    pub fn set_extension(&mut self, extension: String) -> bool {
        if extension.is_empty() {
            return false;
        }
        self.extension = extension;
        true
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn set_content(&mut self, content: Vec<u8>) {
        self.content = content
    }
}
