use std::fs::File as FsFile;
use std::path::{Path, PathBuf};

use crate::project::ProjectBuilder;
use std::io::{Error as IoError, Seek, Write};

#[derive(Debug)]
pub enum ExportError {
    Io(IoError),
    Zip(zip::result::ZipError),
}

impl std::error::Error for ExportError {}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::Io(io) => write!(f, "{io}"),
            ExportError::Zip(zip) => write!(f, "{zip}"),
        }
    }
}

impl From<std::io::Error> for ExportError {
    fn from(value: std::io::Error) -> Self {
        ExportError::Io(value)
    }
}
impl From<zip::result::ZipError> for ExportError {
    fn from(value: zip::result::ZipError) -> Self {
        ExportError::Zip(value)
    }
}

/// Return amount written
pub fn write_zip<W: Write + Seek>(
    writer: W,
    project: ProjectBuilder,
) -> Result<usize, zip::result::ZipError> {
    let mut res_buf = vec![];
    let project = project.build(&mut res_buf);
    let mut zip = zip::ZipWriter::new(writer);
    for mut res in res_buf {
        zip.start_file(
            res.generate_file_name().to_str().unwrap(),
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
        )?;
        let _written = zip.write(&res.content())?;
    }
    zip.start_file(
        PathBuf::from("project")
            .with_extension("json")
            .to_str()
            .unwrap(),
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
    )?;
    let written = zip.write(&serde_json::to_string_pretty(&project).unwrap().into_bytes())?;
    Ok(written)
}

pub fn export<P: AsRef<Path>>(project: ProjectBuilder, path: P) -> Result<(), ExportError> {
    let zip_file = FsFile::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let _written = write_zip(zip_file, project)?;
    Ok(())
}
