use actix_web::{get, web::Json, Responder, Result};
use common::{ApiAboutRespons, ApiResponse};

use crate::VESRION;

fn create_about() -> ApiAboutRespons {
    let contributor = vec![(
        "Yogi Astawan".to_owned(),
        "yogi.astawan@gmail.com".to_owned(),
    )];
    let tech = vec![(
        "Rust Language".to_owned(),
        "htps://rust-lang.org".to_owned(),
        "cc".to_owned(),
    )];

    let contact = vec![("email".to_owned(), "yogi.astawan@gmail.com".to_owned())];
    ApiAboutRespons {
        version: VESRION.to_owned(),
        description: "App to provide ability work".to_owned(),
        contributor,
        tech,
        contact,
    }
}

#[get("/about")]
async fn about() -> Result<impl Responder> {
    let res = ApiResponse {
        title: "About".to_string(),
        content: create_about(),
    };
    Ok(Json(res))
}
