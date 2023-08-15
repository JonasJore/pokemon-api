use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use pokemon_rs;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonResponse {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionResponse {
    id: i32,
    region_name: String,
}

pub fn invalid_req(message: String) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "error": "InvalidArgument",
        "message": message
    }))
}

pub fn capitalize_first_letter(name: String) -> String {
    let mut v: Vec<char> = name.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();

    return v.into_iter().collect();
}

pub fn does_pokemon_exist(name: &String) -> bool {
    let result = std::panic::catch_unwind(|| {
        pokemon_rs::get_id_by_name(name.as_str(), None);
    });

    return result.is_ok();
}

pub fn does_region_exist(number: &usize) -> bool {
    let result = std::panic::catch_unwind(|| {
        pokemon_rs::get_region(*number);
    });

    return result.is_ok();
}

async fn default_handler() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "error": "NotFound", "message": "Invalid path"
    }))
}

#[get("/pokemon-api/id/{id}")]
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

#[get("/pokemon-api/all")]
async fn get_all_pokemon() -> HttpResponse {
    let all_pokemon: Vec<PokemonResponse> = pokemon_rs::get_all(None)
        .iter()
        .map(|&p| PokemonResponse {
            id: pokemon_rs::get_id_by_name(p, None) as i32,
            name: p.to_string(),
        })
        .collect();

    return HttpResponse::Ok().json(json!({ "pokemon": all_pokemon }));
}

#[get("/pokemon-api/number_of_pokemon")]
async fn number_of_pokemon() -> HttpResponse {
    let pokemon_number = pokemon_rs::get_all(None).iter().len() as i32;

    return HttpResponse::Ok().json(json!({
        "numberOfPokemon": pokemon_number.to_string()
    }));
}

#[get("/pokemon-api/random")]
async fn random_pokemon() -> HttpResponse {
    let random = pokemon_rs::random(None);
    let response = PokemonResponse {
        id: pokemon_rs::get_id_by_name(random.as_str(), None) as i32,
        name: random,
    };

    return HttpResponse::Ok().json(json!(response));
}

#[get("/pokemon-api/name/{name}")]
async fn get_pokemon_id_by_name(param: web::Path<(String,)>) -> HttpResponse {
    let url_param = capitalize_first_letter(param.into_inner().0);
    if !does_pokemon_exist(&url_param) {
        return invalid_req(String::from("Pokemon does not exist"));
    }
    let id = pokemon_rs::get_id_by_name(&url_param.as_str(), None);
    return HttpResponse::Ok().json(json!(PokemonResponse {
        id: id as i32,
        name: url_param
    }));
}

#[get("/pokemon-api/region/{region_number}")]
async fn get_region(param: web::Path<(usize,)>) -> HttpResponse {
    let url_param = param.into_inner().0;

    if !does_region_exist(&url_param) {
        return invalid_req(String::from("Region does not exist"));
    }

    let region = pokemon_rs::get_region(url_param);
    return HttpResponse::Ok().json(json!(RegionResponse {
        id: url_param as i32,
        region_name: region
    }));
}

// TODO: getting all regions manually here until pokemon_rs is supporting getting all regions
#[get("/pokemon-api/region/all/")]
async fn get_all_regions() -> HttpResponse {
    let all_regions: Vec<RegionResponse> = (1..10)
        .step_by(1)
        .into_iter()
        .map(|r| {
            return RegionResponse {
                id: r,
                region_name: pokemon_rs::get_region(r as usize),
            };
        })
        .collect();

    return HttpResponse::Ok().json(json!({ "regions": all_regions }));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(get_pokemon_by_id)
            .service(get_all_pokemon)
            .service(get_pokemon_id_by_name)
            .service(get_region)
            .service(get_all_regions)
            .service(random_pokemon)
            .service(number_of_pokemon)
            .default_service(web::route().to(default_handler))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
