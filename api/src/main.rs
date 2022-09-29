use std::env;
use std::sync::RwLock;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2;
use dotenvy::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use api::{bot, routes, states};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("Missing database url");
    let secret_key = env::var("SECRET_KEY").expect("Missing backend secret key");
    let cookie_key = Key::from(secret_key.as_bytes());

    // States init
    let config = states::app_config::AppConfig::new().unwrap();
    let manager = r2d2::ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let twitch_app_credentials = web::Data::new(RwLock::new(
        states::twitch_credentials::TwitchClientCredentials::new(&config).await,
    ));

    let shared_pool = web::Data::new(pool.clone());
    let shared_config = web::Data::new(config.clone());

    // Twitch bot
    let mut bot_manager = bot::manager::BotManager::new(config.clone(), pool.clone());
    if let Err(bot_manager_error) = bot_manager.connect().await {
        log::error!(
            target: bot::LOG_TARGET,
            "Bot cannot connect to Twitch IRC socket: {}",
            &bot_manager_error
        );
    }

    let shared_bot_manager = web::Data::new(RwLock::new(bot_manager));
    let is_local_app = matches!(config.app_env, states::app_config::AppEnv::Local);
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let address = format!("0.0.0.0:{}", port);
    let app_config = config.clone();

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
            .app_data(shared_config.clone())
            .app_data(shared_bot_manager.clone())
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
                    .service(
                        web::scope("/command")
                            .service(routes::command::get_user_commands)
                            .service(routes::command::create_user_command)
                            .service(routes::command::update_user_command)
                            .service(routes::command::delete_user_command),
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
