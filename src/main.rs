use axum::{extract::Form, response::Html, routing::get, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    // build our application with some routes
    let app = Router::new().route("/", get(show_form).post(accept_form));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>
                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>
                    <label>
                        Enter your password:
                        <input type="text" name="password">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    name: String,
    email: String,
    password: String,
}

async fn accept_form(Form(input): Form<Input>) {
    dbg!(&input);
}
