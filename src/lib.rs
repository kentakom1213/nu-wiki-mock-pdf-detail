use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::{
    sync::{Arc, RwLock},
};

const PDF_DATA_JSON: &str = "./data.json";
type Db = Arc<RwLock<Vec<PdfDetail>>>;

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    // pdfの読み込み
    let pdf_data = read_json_file(PDF_DATA_JSON).unwrap();

    // データベースとなるベクタを作成
    let db = make_db(&pdf_data);

    // app
    let app = Router::new()
        .route("/detail/:id", get(get_detail))
        .with_state(db);

    let sync_wrapper = sync_wrapper::SyncWrapper::new(app);

    Ok(sync_wrapper)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PdfDetail {
    file_id: usize,
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PdfDetailData {
    pdf_detail: Vec<PdfDetail>,
}

/// ## read_json_file
/// jsonファイルを読み込む
fn read_json_file(file_path: &str) -> std::io::Result<PdfDetailData> {
    // jsonファイルの読み込み
    let data_json = OpenOptions::new().read(true).open(file_path).unwrap();

    let reader = BufReader::new(&data_json);
    let pdf_data: PdfDetailData = serde_json::from_reader(reader)?;

    Ok(pdf_data)
}

/// ## make_db
/// Jsonからデータベースを作成
fn make_db(data: &PdfDetailData) -> Db {
    let map: Vec<PdfDetail> = data.pdf_detail.clone();
    Arc::new(RwLock::new(map))
}

/// ## get_detail
/// 指定されたIDがデータにあった場合、詳細情報を返す
async fn get_detail(
    Path(id): Path<usize>,
    State(db): State<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    let detail = db
        .read()
        .unwrap()
        .get(id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(detail))
}
