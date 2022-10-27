use std::fs::{self};

use poem::{
    endpoint::EmbeddedFileEndpoint,
    error::NotFoundError,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    web::Path,
    web::{Form, Html, Multipart},
    EndpointExt, IntoResponse, Response, Route, Server,
};

use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "files"]
pub struct Files;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
async fn upload_form_log() -> Html<&'static str> {
    Html(
        r###"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Poem Toy</title>
        </head>
        <body>
            <form action="/upload_log" enctype="multipart/form-data" method="post">
                <input type="file" name="upload" id="file">
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
        "###,
    )
}

#[derive(Deserialize)]
struct FourBoxParams {
    first_name: String,
    last_name: String,
    message_one: String,
    message_two: String,
}

#[handler]
async fn four_box(Form(params): Form<FourBoxParams>) -> impl IntoResponse {
    println!(
        "LOG: first_name={:?} first_name={:?} message_one={:?} message_two={:?}",
        params.first_name, params.last_name, params.message_one, params.message_two
    );
    Response::builder().status(StatusCode::OK).finish()
}

#[handler]
async fn upload_form_save() -> Html<&'static str> {
    Html(
        r###"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Poem Toy</title>
        </head>
        <body>
            <form action="/upload_save" enctype="multipart/form-data" method="post">
                <input type="file" name="upload" id="file">
                <button type="submit">Submit</button>
            </form>
        </body>
        </html>
        "###,
    )
}

#[handler]
async fn upload_log(mut multipart: Multipart) -> &'static str {
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().map(ToString::to_string);
        let file_name = field.file_name().map(ToString::to_string);
        if let Ok(bytes) = field.bytes().await {
            println!(
                "LOG: name={:?} filename={:?} length={}",
                name,
                file_name,
                bytes.len()
            );
        }
    }
    "File uploaded successfully!"
}

#[handler]
async fn upload_save(mut multipart: Multipart) -> &'static str {
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().map(ToString::to_string);
        let file_name = field.file_name().map(ToString::to_string).unwrap();
        if let Ok(bytes) = field.bytes().await {
            let fnc = &file_name.clone();
            let path = std::path::Path::new(fnc);
            fs::write(path, &bytes).expect("Unable to write file");
            println!(
                "SAVE: name={:?} filename={:?} length={}",
                name,
                file_name,
                bytes.len()
            );
        }
    }
    "File uploaded successfully!"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/", EmbeddedFileEndpoint::<Files>::new("index.html"))
        .at(
            "/four_box",
            get(EmbeddedFileEndpoint::<Files>::new("four_box_form.html")).post(four_box),
        )
        .at("/hello/:name", get(hello))
        .at("/upload_log", get(upload_form_log).post(upload_log))
        .at("/upload_save", get(upload_form_save).post(upload_save))
        .catch_error(|_: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Something Went Wrong")
        });

    Server::new(TcpListener::bind("127.0.0.1:2112"))
        .run(app)
        .await
}
