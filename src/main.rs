use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use pokemon_rs;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

async fn invalid(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/pokemon/{id}")]
async fn get_pokemon_by_id(param: web::Path<(usize,)>) -> HttpResponse {
    let pokemon_by_id = pokemon_rs::get_by_id(param.into_inner().0, None);
    return HttpResponse::Ok().body(pokemon_by_id);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_pokemon_by_id)
            // .route("/hey", web::get().to(invalid))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
