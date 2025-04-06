use actix_web::{HttpResponse, Responder, post, web};
use rusty_ytdl::Video;
use serde::Deserialize;
use tracing::debug;
use utoipa::ToSchema;

use crate::{
    error::Error,
    utils::{
        audio::proccess_wav_into_db::proccess_wav_into_db,
        youtube::{
            get_youtube_audio_wav::get_youtube_audio_wav, get_youtube_info::get_youtube_info,
        },
    },
};

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
    let url = req.url.clone();
    debug!("test 1");
    let yt_info = get_youtube_info(&url).await?;
    debug!("test 2");
    let wav = get_youtube_audio_wav(&url).await?;
    debug!("test 4");
    proccess_wav_into_db(&wav, &yt_info).await?;
    debug!("test 5");
    Ok(HttpResponse::Ok().finish())
}
