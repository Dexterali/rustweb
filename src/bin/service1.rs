use axum::{response::Html, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::net::SocketAddr;
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct ShareFile {
    path: String,
    owner: String,
    size: String,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/info", get(list_all))
        .route("/info/id", get(file_info));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1 style='color:blue'>Hello, World!</h1>")
}

async fn list_all() -> impl IntoResponse {
    let mut filenames = HashMap::new();

    // 遍历当前目录
    for entry in WalkDir::new("./src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;
    }

    //let s = serde_json::to_string(&filenames).unwrap();
    //s

    serde_json::to_string(&filenames).unwrap()
}

async fn file_info() -> impl IntoResponse {
    let f: ShareFile = ShareFile {
        path: "tmp/a.rs".to_owned(),
        owner: "李文举".to_owned(),
        size: "4bytes".to_owned(),
    };

    Json(f)
}
