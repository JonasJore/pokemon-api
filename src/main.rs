#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use pokemon_rs as pokemon;

#[get("/<id>")]
fn get_pokemon_by_id(id: usize) -> String {
    return pokemon::get_by_id(id, None).to_string();
}
#[get("/<name>", rank = 1)]
fn get_pokemon_by_name(name: String) -> String {
    let s: usize = pokemon::get_id_by_name(&name, None);
    return s.to_string();
}

fn main() {
    rocket::ignite()
        .mount("/pokemon", routes![get_pokemon_by_id, get_pokemon_by_name])
        .launch();
}
