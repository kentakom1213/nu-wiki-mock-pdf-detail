use serde::{Deserialize, Serialize};

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
pub fn read_list_data(file: &str) -> std::io::Result<PdfList> {
    // データのデシリアライズ
    let pdf_list: PdfList = serde_json::from_str(file)?;

    Ok(pdf_list)
}
