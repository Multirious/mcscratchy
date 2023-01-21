use std::fs::File as FsFile;
use std::io::{Error as IoError, Read, Result as IoResult, Write};
use std::path::{Path, PathBuf};

use super::ProjectBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationError {
    InvalidFileExtension,
}

impl std::error::Error for VerificationError {}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[derive(Debug)]
pub enum BuildError {
    Io(IoError),
    Zip(zip::result::ZipError),
}

impl std::error::Error for BuildError {}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::Io(io) => write!(f, "{io}"),
            BuildError::Zip(zip) => write!(f, "{zip}"),
        }
    }
}

impl From<std::io::Error> for BuildError {
    fn from(value: std::io::Error) -> Self {
        BuildError::Io(value)
    }
}
impl From<zip::result::ZipError> for BuildError {
    fn from(value: zip::result::ZipError) -> Self {
        BuildError::Zip(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub(crate) path: PathBuf,
    pub(crate) content: Vec<u8>,
}

impl Resource {
    pub fn load<P: AsRef<Path>>(path: P) -> IoResult<Resource> {
        let mut file = FsFile::options().read(true).open(&path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let file = Resource {
            path: path.as_ref().to_path_buf(),
            content: buf,
        };
        Ok(file)
    }

    pub fn verify(self) -> Result<ValidResource, VerificationError> {
        let extension = self
            .path
            .extension()
            .ok_or(VerificationError::InvalidFileExtension)?
            .to_string_lossy();
        Ok(ValidResource {
            extension: extension.into_owned(),
            file: self,
        })
    }

    pub fn load_and_verify<P: AsRef<Path>>(
        path: P,
    ) -> Result<Result<ValidResource, VerificationError>, IoError> {
        Self::load(path).map(|ok| ok.verify())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidResource {
    pub(crate) extension: String,
    pub(crate) file: Resource,
}

impl ValidResource {
    pub fn md5_hash(&self) -> [u8; 16] {
        md5::compute(&self.file.content).0
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub struct ProjectFileBuilder {
    project: ProjectBuilder,
    directory: PathBuf,
    project_name: PathBuf,
}

impl ProjectFileBuilder {
    pub fn new(project_builder: ProjectBuilder) -> ProjectFileBuilder {
        ProjectFileBuilder {
            project: project_builder,
            directory: ".".into(),
            project_name: "Scratch Project".into(),
        }
    }

    pub fn path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.directory = path.into();
        self
    }

    pub fn name<P: Into<PathBuf>>(mut self, name: P) -> Self {
        self.project_name = name.into();
        self
    }

    pub fn build(self) -> Result<(), BuildError> {
        let ProjectFileBuilder {
            project,
            directory,
            project_name,
        } = self;
        let mut file_buff = vec![];
        let project = project.build(&mut file_buff);
        let mut zip_file = FsFile::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(directory.join(project_name.with_extension("sb3")))?;
        let mut zip = zip::ZipWriter::new(&mut zip_file);
        for asset_file in file_buff {
            let Resource { path, content } = asset_file;
            zip.start_file(
                path.to_str().unwrap(),
                zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Deflated),
            )?;
            let _written = zip.write(&content)?;
        }
        zip.start_file(
            PathBuf::from("project")
                .with_extension("json")
                .to_str()
                .unwrap(),
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
        )?;
        let _written = zip.write(&serde_json::to_string_pretty(&project).unwrap().into_bytes())?;
        zip.finish()?;
        Ok(())
    }
}
