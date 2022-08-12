use actix_session::Session;
use actix_web::{get, http::header, web, Error, HttpResponse};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::models::user::{self, get_or_create_user};
use crate::states::app_config::AppConfig;
use crate::twitch::id_api;
use crate::types::DbPool;

#[get("/login/authorization")]
pub async fn login_request_to_twitch(
    app_config: web::Data<AppConfig>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let oauth_state = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let authorization_url = id_api::get_authorization_url(&app_config, &oauth_state);
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
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    session: Session,
    authorized_data: Option<web::Query<AuthorizedData>>,
    authorized_error: Option<web::Query<AuthorizedError>>,
) -> Result<HttpResponse, Error> {
    let mb_oauth_state = session.get::<String>("oauth_state")?;
    let db = pool.get().expect("couldn't get db connection from pool");

    let response = match (authorized_data, mb_oauth_state) {
        (Some(query_data), Some(session_oauth_state)) => {
            let AuthorizedData { code, state, .. } = query_data.into_inner();
            if session_oauth_state == state {
                let redirection_response = id_api::get_user_access_token(&app_config, &code)
                    .await
                    .ok()
                    .and_then(|tokens| session.insert("authorization", tokens).ok())
                    // .and_then(|()| {
                    //     twitch::api::get_current_user(, access_token)
                    //
                    // })
                    .and_then(|()| {
                        Some(
                            HttpResponse::Found()
                                .append_header((header::LOCATION, "http://localhost:3000/app"))
                                .finish(),
                        )
                    })
                    .unwrap_or_else(|| HttpResponse::InternalServerError().finish());

                // get_or_create_user(&db, &user).expect("Cannot create/get new twitch user");

                redirection_response
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
