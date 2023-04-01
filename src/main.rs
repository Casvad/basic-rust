#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::associations::HasTable;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::{NewPost, SimplePost};

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("db url variable not found");

    let mut conn = PgConnection::establish(&db_url).expect("cannot connect to database");

    use self::models::{Post, NewPost};
    use self::schema::posts::*;
    use self::schema::posts::dsl::*;

    let new_post = NewPost {
        title: "My first blogpost",
        body: "Lorem",
        slug: "first-post",
    };

    //let post: Post = diesel::insert_into(posts::table()).values(&new_post).get_result(&mut conn).expect("Cannot insert data");

    //select * from posts
    let post_result = posts.load::<Post>(&mut conn).expect("Error running select query");

    //select * from posts limit 1
    let single_post = posts.limit(1).load::<Post>(&mut conn).expect("Error running select query");

    //select title, body from posts limit 1
    let custom_cols = posts.select((title, body)).limit(1).load::<SimplePost>(&mut conn).expect("Error running select query");

    //select * from posts order by id
    let post_result = posts.order(id).load::<Post>(&mut conn).expect("Error running select query");

    //select * from posts order by id desc
    let post_result = posts.order(id.desc()).load::<Post>(&mut conn).expect("Error running select query");

    //select * from posts where id = 2
    let post_result = posts.filter(id.eq(2)).load::<Post>(&mut conn).expect("Error running select query");

    //select * from posts where slug = 'first-post'
    let post_result = posts.filter(slug.eq("first-post")).load::<Post>(&mut conn).expect("Error running select query");

    for post in post_result {
        println!("{:?}", post);
    }

    let post_update = diesel::update(posts.filter(slug.eq("first-post")))
        .set((body.eq("other value"), slug.eq("first-post")))
        .get_results::<Post>(&mut conn);

    diesel::delete(posts.filter(slug.eq("second-post"))).execute(&mut conn).expect("Delete failed");
}
