use actix_web::get;

#[get("/health_check")]
pub async fn health_check() -> &'static str {
    "Health ckeck"
}
