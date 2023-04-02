#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::associations::HasTable;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::{NewPost, NewPostHandler, SimplePost};

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};
use self::models::{Post};
use self::schema::posts::*;
use self::schema::posts::dsl::*;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/posts")]
async fn get_posts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Problems getting connection");

    match web::block(move || { posts.load::<Post>(&mut conn) }).await {
        Ok(data) => {
            HttpResponse::Ok().body("Data found")
        }
        Err(e) => {
            HttpResponse::Ok().body("Error Getting data")
        }
    }
}

#[post("/posts")]
async fn create_posts(
    pool: web::Data<DbPool>,
    item: web::Json<NewPostHandler>,
) -> impl Responder {
    let conn_result = pool.get();
    match conn_result {
        Ok(conn) => {
            let mut conn_box = Box::new(conn);
            let mut_ref_conn = &mut *conn_box;
            match web::block(move || { Post::create_post(mut_ref_conn, &item) }).await {
                Ok(data) => {
                    HttpResponse::Ok().body("Element created")
                }
                Err(e) => {
                    HttpResponse::Ok().body("Error Getting data")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Could not connect to database"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("db url variable not found");
    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(connection).expect("Cannot construct pool");

    HttpServer::new(move || {
        App::new()
            .service(get_posts)
            .service(create_posts)
            .app_data(Data::new(pool.clone()))
    }).bind(("0.0.0.0", 8080)).unwrap().run().await
}

fn example_diesel() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("db url variable not found");

    let mut conn = PgConnection::establish(&db_url).expect("cannot connect to database");

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
