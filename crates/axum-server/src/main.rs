mod api_service;
mod config;
mod errors;
mod multiplex_service;

use crate::api_service::UsersService;
use crate::errors::CustomError;

use axum::{
    body::{self, Body, Empty},
    extract::{Extension, Path},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};

use self::multiplex_service::MultiplexService;

use grpc_api::api::users_server::UsersServer;
use http::{header::CONTENT_TYPE, Request};
use tonic::transport::Server;
use tower::{make::Shared, steer::Steer, BoxError, ServiceExt};

use assets::templates::statics::StaticFile;

use db::User;
use serde::Deserialize;
use std::net::SocketAddr;
use validator::Validate;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    let app = Router::new()
        .route("/", get(users))
        .route("/sign_up", post(accept_form))
        .route("/static/*path", get(static_path))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // grpc apiリクエストハンドル
    let grpc = tonic::transport::Server::builder()
        // .add_service(reflection_service)
        .add_service(tonic_web::enable(UsersServer::new(UsersService { pool })))
        .into_service();

    // Create a service that can respond to Web and gRPC

    let service = MultiplexService::new(app, grpc);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .await
        .unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;
    let users = db::queries::users::get_users().bind(&client).all().await?;

    Ok(Html(ui_components::users::users(users)))
}

#[derive(Deserialize, Validate)]
struct SignUp {
    #[validate(email)]
    email: String,
}

async fn accept_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Response, CustomError> {
    if form.validate().is_err() {
        return Ok((StatusCode::BAD_REQUEST, "Bad resuest").into_response());
    }

    let client = pool.get().await?;
    let email = form.email;

    // Todo - パスワードをハッシュ化する
    let hashed_password = String::from("aaaa");
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str(), &hashed_password.as_str())
        .await?;

    // 303 redirect to user list
    Ok(Redirect::to("/").into_response())
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(body::boxed(Body::from(data.content)))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}
