use actix_web::{get, web};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::states;

#[get("/login")]
pub async fn login_twitch(app_state: web::Data<states::AppState>) -> String {
    let oauth_state = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let credentials = app_state.twitch_credentials.read().unwrap();

    println!("{:?}", &credentials);

    return oauth_state;
}
