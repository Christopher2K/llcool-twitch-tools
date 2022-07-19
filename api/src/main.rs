use dotenvy::dotenv;
use std::sync::RwLock;

#[macro_use]
extern crate rocket;

mod guards;
mod states;
mod twitch;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[get("/login")]
async fn login_to_twitch(
    locked_credentials: &rocket::State<RwLock<states::TwitchClientCredentialsState>>,
    _c: guards::twitch_credentials_guard::TwitchCredentialsGuard,
) -> String {
    let lock = locked_credentials.read().unwrap();
    String::from(&lock.access_token)
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let twitch_app_credentials = states::TwitchClientCredentialsState::new().await;
    let thread_safe_app_credentials = RwLock::new(twitch_app_credentials);

    rocket::build()
        .manage(thread_safe_app_credentials)
        .mount("/", routes![index, login_to_twitch])
}
