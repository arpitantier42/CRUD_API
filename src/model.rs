
use crate::schema::*;

use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Hero {
    pub id: i32, 
    pub fantasy_name: String,
    pub real_name: Option<String>,
    pub spotted_photo: String,
    pub strength_level: i32,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="my_todos"]
pub struct NewHero<'x> {
    pub fantasy_name: &'x str,
    pub real_name: Option<&'x str>,
    pub spotted_photo: String,
    pub strength_level: i32,
}
