use actix_web::{post, web, Result};
use serde::Deserialize;


#[derive(Deserialize)]
struct Req {
    input_file: Vec<u8>,
}

#[post("")]
pub async fn main_route(req: web::Json<Req>) -> Result<String> {


    Ok(format!("File: {:?}!", req.input_file))

}