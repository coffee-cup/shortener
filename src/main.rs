#![feature(plugin, proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

extern crate harsh;
extern crate redis;

mod repository;
mod shortener;

use repository::Cache;
use rocket::{
    http::Status,
    response::{status::Custom, Redirect},
    State,
};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use serde::Deserialize;
use std::{env, sync::RwLock};

use crate::repository::RedisRepository;

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    url: String,
}

#[get("/<id>")]
fn lookup(repo: State<RwLock<RedisRepository>>, id: String) -> Result<Redirect, &'static str> {
    let mut repo = repo.write().unwrap();
    match repo.lookup(&id) {
        Some(url) => Ok(Redirect::to(format!("{}", url))),
        _ => Err("Requested ID was not found."),
    }
}

#[post("/", format = "json", data = "<data>")]
fn shorten(repo: State<RwLock<RedisRepository>>, data: Json<Url>) -> Custom<String> {
    let ref url = format!("{}", data.url);
    if !url.starts_with("https") && !url.starts_with("http") {
        return Custom(Status::BadRequest, format!("not a valid url {:?}", url));
    }
    let mut repo = repo.write().unwrap();
    let id = repo.store(&url);

    let base_url = match env::var("RAILWAY_STATIC_URL") {
        Ok(val) => format!("https://{}", val),
        Err(_e) => "http://localhost:8000".into(),
    };

    return Custom(Status::Ok, format!("{}{}", base_url, id));
}

fn main() {
    let repo = RedisRepository::new().unwrap();

    rocket::ignite()
        .manage(RwLock::new(repo))
        .mount("/", routes![lookup, shorten])
        .mount("/", StaticFiles::from("public"))
        .launch();
}
