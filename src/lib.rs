use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::{Arc, RwLock};

// データの読み込み
mod data;
use crate::data::{PDF_DETAIL_DATA, PDF_LIST_DATA};
mod pdf_detail;
use crate::pdf_detail::{read_detail_data, PdfDetail, PdfDetailData};
mod pdf_list;

type Db = Arc<RwLock<Vec<PdfDetail>>>;

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    // pdfの読み込み
    let pdf_data = read_detail_data(PDF_DETAIL_DATA).unwrap();

    // データベースとなるベクタを作成
    let db = make_db(&pdf_data);

    // app
    let app = Router::new()
        .route("/detail/:id", get(get_detail))
        .with_state(db);

    let sync_wrapper = sync_wrapper::SyncWrapper::new(app);

    Ok(sync_wrapper)
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
