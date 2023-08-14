use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use pokemon_rs;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct PokemonResponse {
    id: i32,
    name: String,
}

pub fn invalid_req(message: String) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "error": "InvalidArgument",
        "message": message
    }))
}

async fn default_handler() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "error": "NotFound", "message": "Invalid path"
    }))
}

#[get("/pokemon/id/{id}")]
async fn get_pokemon_by_id(param: web::Path<(usize,)>) -> HttpResponse {
    let id_from_param: i32 = param.into_inner().0 as i32;
    if id_from_param < 1 || id_from_param > 1008 {
        return invalid_req(String::from("Given id must be a valid pokemon id"));
    }
    let pokemon_by_id = pokemon_rs::get_by_id(id_from_param as usize, None);
    let pokemon_response: PokemonResponse = PokemonResponse {
        id: id_from_param,
        name: pokemon_by_id,
    };

    return HttpResponse::Ok().json(json!(&pokemon_response));
}

#[get("/pokemon/all")]
async fn get_all_pokemon() -> HttpResponse {
    let all_pokemon: Vec<PokemonResponse> = pokemon_rs::get_all(None)
        .iter()
        .map(|&p| PokemonResponse {
            id: pokemon_rs::get_id_by_name(p, None) as i32,
            name: p.to_string(),
        })
        .collect();

    return HttpResponse::Ok().json(json!(&all_pokemon));
}

#[get("/pokemon/number_of_pokemon")]
async fn number_of_pokemon() -> HttpResponse {
    let pokemon_number = pokemon_rs::get_all(None).iter().len() as i32;

    return HttpResponse::Ok().json(json!({
        "numberOfPokemon": pokemon_number.to_string()
    }));
}

#[get("/pokemon/random")]
async fn random_pokemon() -> HttpResponse {
    let random = pokemon_rs::random(None);
    let response = PokemonResponse {
        id: pokemon_rs::get_id_by_name(random.as_str(), None) as i32,
        name: random,
    };

    return HttpResponse::Ok().json(json!(response));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(get_pokemon_by_id)
            .service(get_all_pokemon)
            .service(random_pokemon)
            .service(number_of_pokemon)
            .default_service(web::route().to(default_handler))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
