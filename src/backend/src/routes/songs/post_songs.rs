use actix_web::{HttpResponse, Responder, post, web};
use rusty_ytdl::Video;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    error::Error,
    utils::{
        audio::{
            parse_wav_bytes_into_wav_struct::convert_yt_audio_to_wav,
            proccess_wav_into_db::proccess_wav_into_db,
        },
        youtube::{get_youtube_audio::get_youtube_audio, get_youtube_info::get_youtube_info},
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

    let yt_info = get_youtube_info(&url).await?;
    let raw_yt_audio = get_youtube_audio(&url).await?;

    let wav = convert_yt_audio_to_wav(raw_yt_audio).await?;

    proccess_wav_into_db(&wav, &yt_info).await?;

    Ok(HttpResponse::Ok().finish())
}
