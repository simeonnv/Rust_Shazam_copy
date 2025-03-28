pub mod post_songs;

use actix_web::{
    Scope,
    web::{self},
};

pub fn songs() -> Scope {
    web::scope("/songs").service(post_songs::post_songs)
}
