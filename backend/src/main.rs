pub mod api;

use std::fs;

use actix_web::{get, web::scope, App, Error, HttpRequest, HttpResponse, HttpServer};
use api::about;
use frontend::{ServerApp, ServerAppProps};
use yew::ServerRenderer;

const VESRION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = "TSM ENG Z";

fn server_app_props(url: String) -> Box<dyn FnOnce() -> ServerAppProps + Send> {
    Box::new(move || ServerAppProps { url: url.into() })
}

#[get("/{tail:.*}")]
async fn yew_app_render(req: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html = fs::read_to_string("dist/index.html").unwrap();

    let content = ServerRenderer::<ServerApp>::with_props(server_app_props(req.uri().to_string()))
        .render()
        .await;
    Ok(HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(index_html.replace("<body>", &format!("<body>{}", &content))))
}

// provide data for about content

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(scope("/capi").service(about))
            .service(actix_files::Files::new("/dist", "./dist"))
            .service(yew_app_render)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
