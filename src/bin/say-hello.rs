use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use walkdir::WalkDir;
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_templates=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new()
        .route("/", get(file_list))
        .route("/info/:filename", get(list_info));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn file_list() -> impl IntoResponse {
    let mut filenames = HashMap::new();

    let path = Path::new("C:\\Users\\WenJu\\Desktop\\LeetCode");
    // 遍历当前目录
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;
    }
    // let s = serde_json::to_string(&filenames).unwrap();

    let file_templates = FileListTemplate { filenames };
    HtmlTemplate(file_templates)
}

async fn list_info(extract::Path(filename): extract::Path<String>) -> impl IntoResponse {
    let path = format!(
        "{}{}",
        "C:\\Users\\WenJu\\Desktop\\LeetCode\\".to_owned(),
        filename
    );
    let path = Path::new(&path);
    let mut input = File::open(path).await.unwrap();
    let mut info = String::new();
    input.read_to_string(&mut info).await.unwrap();
    info
}

#[derive(Template)]
#[template(path = "file_list.html")]
struct FileListTemplate {
    filenames: HashMap<String, i32>,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
