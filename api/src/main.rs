use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2;
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use api::{bot, routes, states};

use std::{env, fmt::format, sync::RwLock};

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

    let manager = r2d2::ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let shared_pool = web::Data::new(pool);

    // Twitch bot
    let mut twitch_bot = bot::Bot::new(shared_pool.clone(), app_config.clone());
    let bot_connect_result = twitch_bot.connect().await;
    if let Err(bot_error) = bot_connect_result {
        log::error!(
            target: bot::LOG_TARGET,
            "Initial connection to Twitch socket failed {}",
            &bot_error
        );
    };

    let shared_twitch_bot = web::Data::new(RwLock::new(twitch_bot));
    let is_local_app = matches!(app_config.app_env, states::app_config::AppEnv::Local);
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let address = format!("0.0.0.0:{}", port);

    // CORS
    let app = move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        let cookie_domain = Some(format!(".{}", app_config.domain.clone()));

        App::new()
            .app_data(shared_pool.clone())
            .app_data(twitch_app_credentials.clone())
            .app_data(app_config.clone())
            .app_data(shared_twitch_bot.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), cookie_key.clone())
                    .cookie_domain(cookie_domain)
                    .build(),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(routes::auth::login_request_to_twitch)
                            .service(routes::auth::get_twitch_access_token)
                            .service(routes::auth::logout)
                            .service(routes::auth::me),
                    )
                    .service(
                        web::scope("/bot")
                            .service(routes::bot::join_chat)
                            .service(routes::bot::leave_chat)
                            .service(routes::bot::get_bot_info)
                            .service(routes::bot::connect),
                    )
                    .service(web::scope("/_dev").service(routes::utils::health_check)),
            )
    };

    // START LOCAL APP
    let server = if is_local_app {
        // SSL config
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(
                "./certs/_wildcard.twitchtools.local-key.pem",
                SslFiletype::PEM,
            )
            .unwrap();
        builder
            .set_certificate_chain_file("./certs/_wildcard.twitchtools.local.pem")
            .unwrap();

        HttpServer::new(app).bind_openssl(address, builder)?.run()
    } else {
        HttpServer::new(app).bind(address)?.run()
    };

    server.await
}
