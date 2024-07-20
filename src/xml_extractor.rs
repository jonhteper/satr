use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub struct XmlExtractor;

impl XmlExtractor {
    /// Extrae recursivamente los archivos xml de un archivo directorio.
    pub fn load_all_xmls<P: AsRef<Path>>(path: P) -> Result<Vec<String>, String> {
        let mut xmls = Vec::new();

        for entry in walkdir::WalkDir::new(path) {
            let entry = entry.map_err(|e| e.to_string())?;

            if entry.file_type().is_dir() {
                continue;
            }

            let file_name = entry.file_name().to_string_lossy();

            if file_name.ends_with(".xml") {
                xmls.push(entry.path().to_string_lossy().to_string());
            }

            if file_name.ends_with(".zip") {
                let inner_xmls = Self::extract_from_zip(entry.path())?;
                inner_xmls.iter().for_each(|xml| xmls.push(xml.to_string()));
            }
        }

        Ok(xmls)
    }

    /// Extrae los archivos xml de un archivo zip
    pub fn extract_from_zip<P: AsRef<Path>>(path: P) -> Result<Vec<String>, String> {
        let mut archive = ZipArchive::try_from_path(path)?;
        let xmls = archive.extract_xml_files()?;

        Ok(xmls)
    }
}

struct ZipArchive {
    pub archive: zip::ZipArchive<BufReader<File>>,
}

impl ZipArchive {
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let zip_files = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

        Ok(ZipArchive { archive: zip_files })
    }

    pub fn extract_xml_files(&mut self) -> Result<Vec<String>, String> {
        let mut xmls = Vec::new();

        for i in 0..self.archive.len() {
            let mut file = self.archive.by_index(i).map_err(|e| e.to_string())?;
            let file_name = file.name();

            if file_name.ends_with(".xml") {
                let mut xml = String::new();
                file.read_to_string(&mut xml).map_err(|e| e.to_string())?;
                xmls.push(xml);
            }
        }

        Ok(xmls)
    }
}
