use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
// use std::fs::OpenOptions;
// use std::io::BufReader;
use std::{
    sync::{Arc, RwLock},
};

const PDF_DATA_JSON: &str = r#"{
    "pdf_detail": [
        {
            "file_id": 0,
            "name": "微分積分学1",
            "url": "https://www.nagoya-u.ac.jp/academics/upload_images/5b2f064da9816cd6192d25c3a6d262ae_1.pdf"
        },
        {
            "file_id": 1,
            "name": "線形代数学",
            "url": "https://www.nagoya-u.ac.jp/academics/upload_images/bosyuuyoko.pdf"
        },
        {
            "file_id": 2,
            "name": "システム数学及び演習1",
            "url": "https://ct.nagoya-u.ac.jp/access/content/attachment/2022_0816130/Assignments/42efe7f1-2c6c-4d5d-b4ee-636737ec18c5/082110424_%E7%AC%AC13%E5%9B%9E%E8%AA%B2%E9%A1%8C_%E4%B8%AD%E6%9D%91%E5%84%AA%E4%BD%9C.pdf"
        },
        {
            "file_id": 3,
            "name": "シミュレーション",
            "url": "https://ct.nagoya-u.ac.jp/access/content/attachment/2022_0816130/Assignments/42efe7f1-2c6c-4d5d-b4ee-636737ec18c5/082110424_%E7%AC%AC13%E5%9B%9E%E8%AA%B2%E9%A1%8C_%E4%B8%AD%E6%9D%91%E5%84%AA%E4%BD%9C.pdf"
        },
        {
            "file_id": 4,
            "name": "物理基礎2",
            "url": "https://ct.nagoya-u.ac.jp/access/content/attachment/2022_0816130/Assignments/42efe7f1-2c6c-4d5d-b4ee-636737ec18c5/082110424_%E7%AC%AC13%E5%9B%9E%E8%AA%B2%E9%A1%8C_%E4%B8%AD%E6%9D%91%E5%84%AA%E4%BD%9C.pdf"
        }
    ]
}"#;
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
fn read_json_file(file: &str) -> std::io::Result<PdfDetailData> {
    // jsonファイルの読み込み
    // let data_json = OpenOptions::new().read(true).open(file_path).unwrap();

    // let reader = BufReader::new(&data_json);
    let pdf_data: PdfDetailData = serde_json::from_str(file)?;

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
