use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

const PDF_DATA_JSON: &str = "./data.json";
type Db = Arc<RwLock<Vec<PdfDetail>>>;

#[tokio::main]
async fn main() {
    // pdfの読み込み
    let pdf_data = read_json_file(PDF_DATA_JSON).unwrap();

    // データベースとなるベクタを作成
    let db = make_db(&pdf_data);

    // app
    let app = Router::new()
        .route("/detail/:id", get(get_detail))
        .with_state(db);

    // ホスティング
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
