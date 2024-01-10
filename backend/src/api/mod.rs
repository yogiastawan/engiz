use actix_web::{get, web::Json, Responder, Result};
use common::{ApiAboutRespons, ApiResponse};

use crate::{NAME, VESRION};

const DESC:&str="Application used to organize engineering staffs, and assist engineering staffs in their work so they can work quickly and efficiently.";

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
        name: NAME.to_owned(),
        version: VESRION.to_owned(),
        description: DESC.to_owned(),
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
