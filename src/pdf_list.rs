use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::BufReader, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdfOverview {
    pub file_id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdfList {
    pub data: Vec<PdfOverview>,
}

/// ## read_list_data
/// jsonファイルを読み込む
pub fn read_list_data(file_path: PathBuf) -> std::io::Result<PdfList> {
    // jsonファイルの読み込み
    let data_json = OpenOptions::new().read(true).open(file_path)?;

    let reader = BufReader::new(&data_json);

    // データのデシリアライズ
    let pdf_data: PdfList = serde_json::from_reader(reader)?;

    Ok(pdf_data)
}
