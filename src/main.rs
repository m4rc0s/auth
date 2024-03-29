use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router, TypedHeader, headers::{Authorization, authorization::Bearer},
};
use tower_http::cors::{Any, CorsLayer};
use jwt_simple::prelude::{HS256Key, Duration};
use skytable::{
    actions::Actions,
    Connection
};
use structs::{AuthError, Auth};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::structs::Session;

mod password;
mod structs;
mod jwt;
mod migration;

#[tokio::main]
async fn main() {
    // logging setup
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let app = Router::new()
        .route("/health-check", get(health_check))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .route("/users/authorize", post(authorize))
        .layer(cors);

    info!("listening on 0.0.0.0:3000");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login(Json(payload): Json<structs::Login>) -> (StatusCode, Json<structs::LoginResult>) {
    info!("user data found: {:?}", payload);

    let mut conn = Connection::new("auth-db", 2003).unwrap();
    let user_data: String = conn.get(payload.username).ok().unwrap();

    info!("user data found: {:?}", user_data);

    let user: structs::User = serde_json::from_str(&user_data).unwrap();
    let result = password::verify(payload.password, user.password);

    info!("generated result is: {:?}", result);

    let key = HS256Key::generate();
    let token = jwt::token(&key).unwrap();
    let session = Session { key: key.to_bytes() };
    conn.set(&token, session).unwrap();

    if result {
        return (
            StatusCode::OK,
            Json(structs::LoginResult {
                token: token,
            }),
        );
    }

    return (
        StatusCode::NOT_ACCEPTABLE,
        Json(structs::LoginResult {
            token: ("".to_string()),
        }),
    );
}

async fn create_user(Json(payload): Json<structs::User>) -> (StatusCode, Json<structs::User>) {
    let user = structs::User {
        username: payload.username,
        password: password::hash(payload.password),
        verified: payload.verified,
    };

    let mut conn = Connection::new("auth-db", 2003).unwrap();

    info!("creating new user...{:?}", user);

    conn.set(&user.username, serde_json::to_string(&user).unwrap())
        .unwrap();

    info!("new user created {:?}", user);

    (StatusCode::CREATED, Json(user))
}

async fn authorize(TypedHeader(authorization): TypedHeader<Authorization<Bearer>>) -> Result<StatusCode, (StatusCode, Json<AuthError>)> {
    let mut conn = Connection::new("auth-db", 2003).unwrap();

    let token = authorization.token();

    let session: Session = conn.get(token).unwrap();

    let result = jwt::verify(&HS256Key::from_bytes(&session.key), token);

    if result {
        Ok(StatusCode::OK)
    } else {
        let auth_error: AuthError = AuthError { message: "Invalid token".to_string() };
        Err((StatusCode::UNAUTHORIZED, Json(auth_error)))
    }
}


async fn health_check() -> &'static str {
    "running"
}
