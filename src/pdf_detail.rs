use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::BufReader, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdfDetail {
    pub file_id: usize,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfDetailData {
    pub data: Vec<PdfDetail>,
}

/// ## read_json_file
/// jsonファイルを読み込む
pub fn read_detail_data(file_path: PathBuf) -> std::io::Result<PdfDetailData> {
    // jsonファイルの読み込み
    let data_json = OpenOptions::new().read(true).open(file_path)?;

    let reader = BufReader::new(&data_json);

    // データのデシリアライズ
    let pdf_data: PdfDetailData = serde_json::from_reader(reader)?;

    Ok(pdf_data)
}
