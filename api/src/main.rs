use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use diesel::r2d2;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

use api::{middlewares, routes, states};

use std::{env, sync::RwLock};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("Missing database url");
    let secret_key = env::var("SECRET_KEY").expect("Missing backend secret key");
    let cookie_key = Key::from(secret_key.as_bytes());

    // States init
    let app_config = web::Data::new(states::app_config::AppConfig::new().unwrap());
    let twitch_app_credentials = web::Data::new(RwLock::new(
        states::twitch_credentials::TwitchClientCredentials::new(&app_config).await,
    ));

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(twitch_app_credentials.clone())
            .app_data(app_config.clone())
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                cookie_key.clone(),
            ))
            .wrap(middlewares::twitch_client_credentials::TwitchClientCredentialsMiddlewareFactory)
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(routes::auth::login_request_to_twitch)
                            .service(routes::auth::get_twitch_access_token),
                    )
                    .service(web::scope("/_dev").service(routes::utils::health_check)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
