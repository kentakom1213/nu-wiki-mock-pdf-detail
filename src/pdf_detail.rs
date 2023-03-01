use serde::{Deserialize, Serialize};

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
pub fn read_detail_data(file: &str) -> std::io::Result<PdfDetailData> {
    // データのデシリアライズ
    let pdf_data: PdfDetailData = serde_json::from_str(file)?;

    Ok(pdf_data)
}
