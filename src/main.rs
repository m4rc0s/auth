use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use skytable::{
    actions::Actions,
    Connection
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod password;
mod structs;


#[tokio::main]
async fn main() {
    // logging setup
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new()
        .route("/health-check", get(health_check))
        .route("/users", post(create_user))
        .route("/users/login", post(login));

    info!("listening on 0.0.0.0:3000");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login(Json(payload): Json<structs::Login>) -> (StatusCode, Json<structs::LoginResult>) {
    info!("user data found: {:?}", payload);

    let mut conn = Connection::new("127.0.0.1", 2003).unwrap();
    let user_data: String = conn.get(payload.username).ok().unwrap();

    info!("user data found: {:?}", user_data);

    let user: structs::User = serde_json::from_str(&user_data).unwrap();

    let result = password::verify(payload.password, user.password);

    if result {
        return (
            StatusCode::OK,
            Json(structs::LoginResult {
                token: "token".to_string(),
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

    let mut conn = Connection::new("127.0.0.1", 2003).unwrap();

    info!("creating new user...{:?}", user);

    conn.set(&user.username, serde_json::to_string(&user).unwrap())
        .unwrap();

    info!("new user created {:?}", user);

    (StatusCode::CREATED, Json(user))
}

async fn health_check() -> &'static str {
    "running"
}


