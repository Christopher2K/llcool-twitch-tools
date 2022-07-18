use dotenvy::dotenv;

#[macro_use]
extern crate rocket;

mod state;
mod twitch;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[get("/login")]
async fn login_to_twitch(
    credentials: &rocket::State<state::TwitchClientCredentialsState>,
) -> &'static str {
    println!("{}", credentials.access_token);
    "Hello"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let twitch_app_credentials = state::TwitchClientCredentialsState::new().await;

    rocket::build()
        .manage(twitch_app_credentials)
        .mount("/", routes![index, login_to_twitch])
}
