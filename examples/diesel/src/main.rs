#[macro_use]
extern crate diesel;

mod db;
mod models;
mod schema;

use itconfig::config;
use dotenv::dotenv;
use diesel::prelude::*;
use crate::models::*;


config! {
    DATABASE_URL,
}


fn main() {
    dotenv().ok();
    config::init();

    let connection = db::establish_connection();
    let posts = get_posts(&connection);

    println!("Displaying {} posts", posts.len());
    for post in posts {
        print!("\n");
        println!("{}", post.title);
        println!("----------");
        println!("{}", post.body);
    }
}


fn get_posts(connection: &PgConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    posts.filter(published.eq(true))
        .limit(5)
        .get_results::<Post>(connection)
        .expect("Error loading posts")
}