use std::fs::{self};

use poem::{
    error::NotFoundError,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    web::{Html, Multipart, Path},
    EndpointExt, Response, Route, Server,
};

#[handler]
async fn index() -> Html<&'static str> {
    Html(
        r###"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Poem Toy</title>
        </head>
        <body>
            <p>
                <h1>Poem Web Framework Demo App</h1>
            </p>
            <p>
                <h3>Say Hello...</h3>
                <p>
                    Anything sent to /hello/anything will say hello to anything
                    so long as its one word. For example,
                    <a href=/hello/elliot>Say Hello to Elliot</a>.
                </p>
            </p>
            <p>
                <h3>Upload a File...</h3>
                <p>
                    Go to <a href='/upload_log'>/upload_log</a> to upload a file and
                    have its stats detailed in the log.  Alternatively, go to
                    <a href='/upload_save'>/upload_save</a> to have the file
                    written to the file system.

                </p>
            </p>
            <p>
                <h3></h3>
                <p>
                </p>
            </p>
            <p>
                <h3></h3>
                <p>
                </p>
            </p>
        </body>
        </html>
        "###,
    )
}

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
        .at("/", get(index))
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

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     if std::env::var_os("RUST_LOG").is_none() {
//         std::env::set_var("RUST_LOG", "poem=debug");
//     }
//     tracing_subscriber::fmt::init();
//     let app = Route::new().nest(
//         "/",
//         //StaticFilesEndpoint::new("./poem/static-files/files").show_files_listing(),
//         StaticFilesEndpoint::new("./files").show_files_listing(),
//     );
//     Server::new(TcpListener::bind("127.0.0.1:3000"))
//         .run(app)
//         .await
// }
