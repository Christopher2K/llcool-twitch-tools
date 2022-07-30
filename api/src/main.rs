use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;

use std::{env, sync::RwLock};

mod middlewares;
mod models;
mod routes;
mod states;
mod twitch;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let secret_key = env::var("SECRET_KEY").expect("Missing backend secret key");
    let cookie_key = Key::from(secret_key.as_bytes());

    let twitch_app_credentials = states::TwitchClientCredentials::new().await;
    let app_data = web::Data::new(states::AppState {
        twitch_credentials: RwLock::new(twitch_app_credentials),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                cookie_key.clone(),
            ))
            .wrap(middlewares::twitch_client_credentials::TwitchClientCredentialsMiddlewareFactory)
            .service(
                web::scope("/api")
                    .service(web::scope("/auth").service(routes::auth::login_twitch))
                    .service(web::scope("/_dev").service(routes::utils::health_check)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
