use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(Files::new("/static", "./static"))
            // Add a route to serve your index.html
            .route("/index.html", web::get().to(index_html))
            .route("/demo.html", web::get().to(demo_html))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index_html() -> HttpResponse {
    // Read the contents of the index.html file and return it as the response
    let index_html_content = include_str!("./templates/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(index_html_content)
}

async fn demo_html() -> HttpResponse {
    // Read the contents of the index.html file and return it as the response
    let demo_html_content = include_str!("./templates/demo.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(demo_html_content)
}
