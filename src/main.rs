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

#[derive(RustEmbed)]
#[folder = "files"]
pub struct Files;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[derive(Deserialize)]
struct ModuleFiveParams {
    merchant_id: String,
    store_name: String,
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[handler]
async fn module_five(Form(params): Form<ModuleFiveParams>) -> impl IntoResponse {
    println!(
        "LOG: merchant_id={:?} store_name={:?} street={:?} city={:?}, state={:?}, zip={:?}",
        params.merchant_id, params.store_name, params.street, params.city, params.state, params.zip
    );
    Response::builder().status(StatusCode::OK).finish()
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
            "/module_five",
            get(EmbeddedFileEndpoint::<Files>::new("module_five.html")).post(module_five),
        )
        .at("/hello/:name", get(hello))
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
