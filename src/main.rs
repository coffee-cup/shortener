#![feature(plugin, proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

extern crate harsh;

mod repository;
mod shortener;

use repository::Repository;
use rocket::{response::Redirect, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    url: String,
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
      POST /
          Ex: curl -H 'Content-Type: application/json' --data '{\"url\": \"https://example.com\"}' http://localhost:8000
          It should respond with a shortened url like http://localhost:8000/gY
      GET /<id>
          Redirects to shortened link. Try from browser or using the example below.
          Ex: curl -i http://localhost:8000/gY
    "
}

#[get("/<id>")]
fn lookup(repo: State<RwLock<Repository>>, id: String) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().lookup(&id) {
        Some(url) => Ok(Redirect::permanent(format!("{}", url))),
        _ => Err("Requested ID was not found."),
    }
}

#[post("/", format = "json", data = "<data>")]
fn shorten(repo: State<RwLock<Repository>>, data: Json<Url>) -> Result<String, String> {
    let ref url = format!("{}", data.url);
    if !url.starts_with("https") && !url.starts_with("http") {
        return Err(format!("Not a valid URL {:?}", url));
    }
    let mut repo = repo.write().unwrap();
    let id = repo.store(&url);
    Ok(format!("http://localhost:8000/{}", id))
}

fn main() {
    rocket::ignite()
        .manage(RwLock::new(Repository::new()))
        .mount("/", routes![index, lookup, shorten])
        .launch();
}
