use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

// データの読み込み
mod data;
use crate::data::{PDF_DETAIL_DATA, PDF_LIST_DATA};
mod pdf_list;
use pdf_list::{read_list_data, PdfOverview};
mod pdf_detail;
use crate::pdf_detail::{read_detail_data, PdfDetail};

// dbの定義
type DbPdfList = Arc<RwLock<Vec<PdfOverview>>>;
type DbPdfDetail = Arc<RwLock<Vec<PdfDetail>>>;

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    // データの読み込み
    let pdf_list = read_list_data(PDF_LIST_DATA).unwrap();
    let pdf_detail = read_detail_data(PDF_DETAIL_DATA).unwrap();

    // データベースとなるベクタを作成
    let db_list = Arc::new(RwLock::new(pdf_list.data));
    let db_detail = Arc::new(RwLock::new(pdf_detail.data));

    // app
    let app = Router::new()
        .route("/list", get(get_list))
        .with_state(db_list)
        .route("/detail/:id", get(get_detail))
        .with_state(db_detail);

    let sync_wrapper = sync_wrapper::SyncWrapper::new(app);

    Ok(sync_wrapper)
}

/// ## get_list
/// pdfの一覧を返す
async fn get_list(State(db): State<DbPdfList>) -> Result<impl IntoResponse, StatusCode> {
    let list = db.read().unwrap().deref().clone();

    Ok(Json(list))
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
