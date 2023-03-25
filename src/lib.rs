use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::{
    ops::Deref,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tower_http::services::ServeFile;

// データの読み込み
mod pdf_list;
use pdf_list::{read_list_data, PdfOverview};
mod pdf_detail;
use crate::pdf_detail::{read_detail_data, PdfDetail};

// ファイルパス
const INDEX_HTML: &str = "index.html";
const LIST_DATA_JSON: &str = "pdf_list.json";
const DETAIL_DATA_JSON: &str = "pdf_detail.json";

// dbの定義
type DbPdfList = Arc<RwLock<Vec<PdfOverview>>>;
type DbPdfDetail = Arc<RwLock<Vec<PdfDetail>>>;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    // データの読み込み
    let pdf_list = read_list_data(static_folder.join(LIST_DATA_JSON))
        .expect("Jsonデータが読み取れませんでした");
    let pdf_detail = read_detail_data(static_folder.join(DETAIL_DATA_JSON))
        .expect("Jsonデータが読み取れませんでした。");

    // データベースとなるベクタを作成
    let db_list = Arc::new(RwLock::new(pdf_list.data));
    let db_detail = Arc::new(RwLock::new(pdf_detail.data));

    // app
    let app = Router::new()
        .route_service("/", ServeFile::new(static_folder.join(INDEX_HTML)))
        .route("/list", get(get_list))
        .with_state(db_list)
        .route("/detail/:id", get(get_detail))
        .with_state(db_detail);

    Ok(app.into())
}

/// ## get_list
/// pdfの一覧を返す
async fn get_list(State(db): State<DbPdfList>) -> Json<Vec<PdfOverview>> {
    let list = db.read().unwrap().deref().clone();

    Json(list)
}

/// ## get_detail
/// 指定されたIDがデータにあった場合、詳細情報を返す
async fn get_detail(
    Path(id): Path<usize>,
    State(db): State<DbPdfDetail>,
) -> Result<impl IntoResponse, StatusCode> {
    let detail = db
        .read()
        .unwrap()
        .get(id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(detail))
}
