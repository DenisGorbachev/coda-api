use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::io;
use std::io::{stdout, BufWriter, Write};
use std::path::{Path, PathBuf};
use stub_macro::stub;

fn main() -> io::Result<()> {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let current_file_path = stub!(PathBuf, "Get the path of the current file at build time");
    let stem = current_file_path.file_stem().expect("stem should exist");
    let parent = current_file_path.parent().expect("parent should exist");
    let context_dir = parent.join(stem);
    writer.write_all(include_bytes!("./fix-build.md"))?;
    let _files = include_dir(&context_dir).into_iter().for_each(|contents| {
        let string = quick_xml::se::to_string(&contents).expect("serialize should succeed");
        writer
            .write_all(string.as_bytes())
            .expect("write should succeed");
    });
    let _files = include_dir_string(&context_dir);
    Ok(())
}

fn include_dir_string(dir: &Path) -> String {
    let files = include_dir(dir);
    files
        .into_iter()
        .map(|_file| stub!(String, "Convert the file to an XML string"))
        .join("\n")
}

fn include_dir(_dir: &Path) -> Vec<FileData> {
    todo!()
}

#[macro_export]
macro_rules! include_file {
    ($path:literal) => {
        let contents = include_str!($path);
        FileData::new($path, contents)
    };
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct FileData {
    path_buf: PathBuf,
    contents: String,
}

impl FileData {}

impl TryFrom<PathBuf> for FileData {
    type Error = io::Error;

    fn try_from(path_buf: PathBuf) -> io::Result<Self> {
        let contents = read_to_string(path_buf.as_path())?;
        Ok(Self {
            path_buf,
            contents,
        })
    }
}

impl TryFrom<&str> for FileData {
    type Error = io::Error;

    fn try_from(path_str: &str) -> Result<Self, Self::Error> {
        let path_buf = PathBuf::from(path_str);
        Self::try_from(path_buf)
    }
}
