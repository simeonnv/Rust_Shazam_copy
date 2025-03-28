use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::{Modify, ToSchema};

use crate::routes;

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

#[derive(OpenApi)]
#[openapi(paths(routes::songs::post_songs::post_songs,), tags())]
pub struct ApiDoc;
