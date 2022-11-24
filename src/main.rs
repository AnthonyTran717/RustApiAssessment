use axum::{extract::Form, response::{Html, Redirect, IntoResponse}, routing::get, Router, Extension};
use credential_storage::CredentialStorage;
use serde::Deserialize;
use std::{net::SocketAddr, sync::{Arc, Mutex}};

mod credential_storage;

#[tokio::main]
async fn main() {
    let credential_storage = Arc::new(Mutex::new(CredentialStorage::new()));

    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    // build our application with some routes
    let app = Router::new().route("/", get(show_form).post(accept_form)).layer(Extension(credential_storage))
        .route("/login", get(show_login).post(accept_login))
        .route("/hello", get(show_hello_word));

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
                        <input type="text" name="username">
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

async fn show_login() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post">
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

async fn show_hello_word() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                Hello World!
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Login {
    email: String,
    password: String,
}

async fn accept_form(Extension(credential_storage): Extension<Arc<Mutex<CredentialStorage>>>, Form(input): Form<Input>) -> impl IntoResponse {
    dbg!(&input);
    let mut credential_storage = credential_storage.lock().unwrap();
    credential_storage.add_user(input.email, input.username, input.password);
    Redirect::to("/hello")
}

async fn accept_login(Extension(credential_storage): Extension<Arc<Mutex<CredentialStorage>>>, Form(login): Form<Login>) -> impl IntoResponse {
    dbg!(&login);
    let credential_storage = credential_storage.lock().unwrap();
    let user = credential_storage.get_user(login.email);
    match user {
        Some(user) => format!("Welcome {}!", user.user),
        None => String::from("No user found"),
    }
}
