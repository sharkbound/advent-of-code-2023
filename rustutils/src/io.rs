use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub struct FileBuilder {
    path: PathBuf,
    options: OpenOptions,
}

impl FileBuilder {
    pub fn from_pathbuf(path: PathBuf) -> Self {
        Self {
            path,
            options: OpenOptions::new(),
        }
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        Some(Self {
            path: match PathBuf::from_str(match path.to_str() {
                Some(s) => s,
                None => return None,
            }) {
                Ok(s) => s,
                Err(_) => return None,
            },
            options: OpenOptions::new(),
        })
    }

    pub fn from_str(path: &str) -> Option<Self> {
        Some(Self {
            path: match PathBuf::from_str(path) {
                Ok(s) => s,
                Err(_) => return None,
            },
            options: OpenOptions::new(),
        })
    }

    pub fn read(&mut self) -> &mut Self {
        self.options.read(true);
        self
    }

    pub fn write(&mut self) -> &mut Self {
        self.options.write(true);
        self
    }

    pub fn append(&mut self) -> &mut Self {
        self.options.append(true);
        self
    }

    pub fn create(&mut self) -> &mut Self {
        self.options.create(true);
        self
    }

    pub fn create_new(&mut self) -> &mut Self {
        self.options.create_new(true);
        self
    }

    pub fn buffered_reader(&self) -> Result<BufReader<File>, Box<dyn Error>> {
        Ok(BufReader::new(
            self.options.open(self.path.clone()).map_err(Box::new)?,
        ))
    }

    pub fn buffered_writer(&self) -> Result<BufWriter<File>, Box<dyn Error>> {
        Ok(BufWriter::new(
            self.options.open(self.path.clone()).map_err(Box::new)?,
        ))
    }

    pub fn file(&self) -> Result<File, Box<std::io::Error>> {
        self.options.open(self.path.clone()).map_err(Box::new)
    }
}
