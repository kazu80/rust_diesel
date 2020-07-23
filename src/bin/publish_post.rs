extern crate sample02_db;
extern crate diesel;

use self::diesel::prelude::*;
use self::sample02_db::*;
use std::env::args;

use diesel::RunQueryDsl;
use sample02_db::models::Post;

fn main() {
    use sample02_db::schema::posts::dsl::{posts, published, id};

    let update_id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");

    let connection = establish_connection();

    diesel::update(posts.find(update_id))
        .set(published.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", update_id));

    let post: Post = posts.filter(id.eq(update_id)).first(&connection).unwrap();

    println!("Published post {}", post.body);
}