#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;
use rocket_contrib::templates::Template
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

pub mod my_todos;

pub mod model;

pub mod schema;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/imgs/<file..>")]
fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("imgs/").join(file)).ok()
}


fn main() {
    rocket::ignite().mount("/", routes![
        assets,
        my_todos::list, 
        my_todos::new, 
        my_todos::insert,
        my_todos::update,
        my_todos::process_update,
        my_todos::delete
        ]).attach(Template::fairing()).launch();
}
