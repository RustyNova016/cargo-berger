use std::fs::File;
use std::io::Read as _;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use color_eyre::eyre::Context as _;
use toml_edit::DocumentMut;

use crate::ColEyre;
use crate::ColEyreVal;

pub mod patch;
pub mod workspace;

pub struct CargoFile {
    file_path: PathBuf,
    doc: Mutex<DocumentMut>,
}

impl CargoFile {
    pub fn load_or_create(path: PathBuf) -> ColEyreVal<Self> {
        if !path.exists() {
            File::create(&path)?;
        }

        Self::load(path)
    }

    pub fn load(path: PathBuf) -> ColEyreVal<Self> {
        let mut file = File::open(&path)
            .context("Couldn't open the cargo config file. Make sure it exists")?;

        let mut data = String::new();
        file.read_to_string(&mut data)
            .context("Couldn't read the cargo config file")?;

        Ok(Self {
            doc: Mutex::new(data.parse()?),
            file_path: path,
        })
    }

    pub fn save(&self) -> ColEyre {
        let mut file = File::create(&self.file_path)
            .context("Couldn't open the berger config file. Make sure it exists")?;
        let doc = self.doc.lock().unwrap();

        write!(file, "{}", doc)?;
        Ok(())
    }
}
