extern crate sample02_db;
extern crate diesel;

use self::sample02_db::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use sample02_db::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
