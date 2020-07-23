extern crate sample02_db;
extern crate diesel;

use self::diesel::prelude::*;
use std::env::args;
use sample02_db::establish_connection;

fn main () {
    use sample02_db::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted)
}