use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{bill::Bill, xml_extractor::XmlExtractor};

#[inline]
fn assets_path() -> std::path::PathBuf {
    let path_str = format!("{}/.assets", env!("CARGO_MANIFEST_DIR"));

    PathBuf::from(&path_str)
}

#[test]
fn deserialize_works() {
    let mut file_path = assets_path();
    println!("{:#?}", &file_path);
    file_path.push("factura.xml");

    let file = std::fs::read_to_string(file_path).expect("Error al leer el archivo XML");
    let bill: Bill = quick_xml::de::from_str(&file).expect("Error al deserializar el XML");

    println!("{:#?}", bill);
}

#[test]
fn zip_extract_works() {
    let mut file_path = assets_path();
    file_path.push("factura.zip");

    let files_content =
        XmlExtractor::extract_from_zip(file_path).expect("Error al extraer los  archivos XML");

    assert!(files_content.len() > 0);
}

#[derive(Deserialize)]
struct Record {
    timestamp: NaiveDateTime,
}

#[test]
fn date_time_deserialize_works() {
    let data = r#"{"timestamp": "2024-04-05T18:09:06"}"#;

    let record: Record = serde_json::from_str(data).unwrap();

    println!("{:?}", record.timestamp);
}

#[test]
fn date_time_parse_works() {
    let datetime_str = "2024-04-05T18:09:06";
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S")
        .expect("Error al parsear la fecha");

    println!("{:?}", datetime);
}
