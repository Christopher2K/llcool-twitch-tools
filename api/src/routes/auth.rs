use std::env;

use actix_session::Session;
use actix_web::{get, http::header, web::Query, Error, HttpResponse};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::twitch;

#[get("/login/authorization")]
pub async fn login_request_to_twitch(session: Session) -> Result<HttpResponse, Error> {
    let twitch_api =
        twitch::TwitchApiConfig::new().expect("Chris: you should make this a shared state");

    let oauth_state = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let authorization_url = twitch_api.get_authorization_url(&oauth_state);
    session.insert("oauth_state", &oauth_state)?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, authorization_url))
        .finish())
}

#[derive(Deserialize)]
pub struct AuthorizedData {
    pub code: String,
    pub scope: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizedError {
    pub error: String,
    pub error_description: String,
    pub state: String,
}

#[get("/login/authorized")]
pub async fn get_twitch_access_token(
    session: Session,
    authorized_data: Option<Query<AuthorizedData>>,
    authorized_error: Option<Query<AuthorizedError>>,
) -> Result<HttpResponse, Error> {
    let mb_oauth_state = session.get::<String>("oauth_state")?;
    let twitch_api =
        twitch::TwitchApiConfig::new().expect("Chris: you should make this a shared state");

    let response = match (authorized_data, mb_oauth_state) {
        (Some(query_data), Some(session_oauth_state)) => {
            let AuthorizedData { code, state, .. } = query_data.into_inner();
            if session_oauth_state == state {
                twitch_api
                    .get_user_access_token(&code)
                    .await
                    .ok()
                    .and_then(|tokens| session.insert("authorization", tokens).ok())
                    .and_then(|()| Some(HttpResponse::Found().append_header((header::LOCATION, "http://localhost:3000/app")).finish()))
                    .unwrap_or_else(|| HttpResponse::InternalServerError().finish())
                // HttpResponse::Ok().finish()
            } else {
                // LOG: State does not correspond
                HttpResponse::InternalServerError().finish()
            }
        }
        _ => {
            if let Some(error) = authorized_error {
                // LOG: OAuth Error
                println!("{:?}", &error)
            } else {
                // LOG: Not supposed to be in this state
            }

            HttpResponse::InternalServerError().finish()
        }
    };

    Ok(response)
}
