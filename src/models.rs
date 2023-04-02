use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct SimplePost {
    pub title: String,
    pub body: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String
}

impl Post {

    fn slugify(text: &str) -> String {

        return text.replace(" ", "-").to_lowercase().clone()
    }

    pub fn create_post<'a>(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        post: &NewPostHandler
    ) -> Result<Post, diesel::result::Error>{

        let slug = Post::slugify(&post.title);

        let new_post = NewPost {
            title: &post.title.clone(),
            body: &post.body.clone(),
            slug: &slug,
        };

        diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn)
    }
}