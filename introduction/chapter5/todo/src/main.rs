use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug)]
struct TodoEntry {
    id: i32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[derive(Deserialize)]
struct DeleteParams {
    id: i32,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        Ok(TodoEntry {
            id: row.get(0)?,
            text: row.get(1)?,
        })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    let html = IndexTemplate { entries };
    let response_body = html.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response_body))
}

#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("INSERT INTO todo (text) VALUES (?)", [&params.text])?;

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish())
}

#[post("/delete")]
async fn delete_todo(
    params: web::Form<DeleteParams>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todo WHERE id = ?", [&params.id])?;

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to create connection pool");

    {
        let conn = pool.get().expect("Failed to get connection from pool");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL
            )",
            params![],
        )
        .expect("Failed to create table");
    }

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .service(index)
            .service(add_todo)
            .service(delete_todo)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
// http://localhost:8080/
