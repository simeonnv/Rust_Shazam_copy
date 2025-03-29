use actix_web::{HttpResponse, Responder, post, web};
use rusty_ytdl::Video;
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
    let video_url = "https://www.youtube.com/watch?v=FZ8BxMU3BYc";
    let video = Video::new(video_url).unwrap(); // Convert error to custom Error type

    let stream = video.stream().await.unwrap();

    // Collect chunks into a String
    // while let Some(chunk) = stream.chunk().await.unwrap() {
    //     println!("{:#?}", chunk);
    // }

    // Optional: Get video info if needed
    let video_info = video.get_info().await.unwrap();
    dbg!(video_info);

    // Return the collected string in the response
    Ok(HttpResponse::Ok().finish())
}
