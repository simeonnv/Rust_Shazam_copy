use actix_web::{HttpResponse, Responder, post, web};
use rustube::Video;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::error::Error;

#[derive(Deserialize, ToSchema)]
pub struct Req {
    pub url: String,
}

#[utoipa::path(
    post,
    path = "/songs",
    responses(),
    request_body = Req,
    tag = "Songs"
)]
#[post("")]
pub async fn post_songs(req: web::Json<Req>) -> Result<HttpResponse, Error> {
    let video = Video::from_url(&req.url.parse()?).await?;
    println!(
        "downloaded video to {:?}",
        rustube::download_best_quality(&req.url).await.unwrap()
    );
    dbg!(video);
    Ok(HttpResponse::Ok().finish())
}
